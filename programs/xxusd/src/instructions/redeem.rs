use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

use crate::error::XxusdError;
use crate::state::{Controller, KaminoDepository};
use crate::utils::maths::checked_sub;
use crate::{CONTROLLER_NAMESPACE, JUPSOL_MINT_PUBKEY};
use crate::core::{Amount, u64_to_amount, safe_u128_to_u64};

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump,
        has_one = redeemable_mint @XxusdError::InvalidRedeemableMint
    )]
    pub controller: Box<Account<'info, Controller>>,

    #[account(mut)]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    #[account(
        constraint = collateral_mint.key() == JUPSOL_MINT_PUBKEY @XxusdError::InvalidCollateralMint
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = redeemable_mint,
        associated_token::authority = user,
    )]
    pub user_redeemable: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = collateral_mint,
        associated_token::authority = user,
    )]
    pub user_collateral: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"kamino_depository"],
        bump,
        has_one = controller @XxusdError::InvalidController,
        has_one = collateral_mint @XxusdError::InvalidCollateralMint,
    )]
    pub kamino_depository: Box<Account<'info, KaminoDepository>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
    // 驗證可贖回金額
    if redeemable_amount.value() == 0 {
        return Err(XxusdError::InvalidRedeemableAmount.into());
    }

    if ctx.accounts.user_redeemable.amount < redeemable_amount.value() {
        return Err(XxusdError::InsufficientRedeemableBalance.into());
    }

    // 銷毀 xxUSD
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                from: ctx.accounts.user_redeemable.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        redeemable_amount.value(),
    )?;

    // 從 Kamino 存儲庫轉移 jupSOL 到用戶
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.kamino_depository.to_account_info(),
                to: ctx.accounts.user_collateral.to_account_info(),
                authority: ctx.accounts.kamino_depository.to_account_info(),
            },
            &[&[
                b"kamino_depository",
                &[ctx.bumps.kamino_depository],
            ]],
        ),
        redeemable_amount.value(), // 假設 1:1 兌換，實際情況可能需要更複雜的計算
    )?;

    // 更新狀態
    let controller = &mut ctx.accounts.controller;
    let current_supply = Amount::from_u128(controller.get_redeemable_circulating_supply())?;
    let new_supply = checked_sub(current_supply, redeemable_amount)?;
    controller.set_redeemable_circulating_supply(new_supply.to_u128())?;

    let kamino_depository = &mut ctx.accounts.kamino_depository;
    let current_amount_under_management = u64_to_amount(kamino_depository.redeemable_amount_under_management.try_into().unwrap());
    let new_amount_under_management = checked_sub(current_amount_under_management, redeemable_amount)?;
    kamino_depository.redeemable_amount_under_management = safe_u128_to_u64(new_amount_under_management.to_u128())?.into();

    Ok(())
}