use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Burn, Transfer};

use crate::error::XxusdError;
use crate::state::{Controller, KaminoDepository, Amount};
use crate::utils::maths::checked_sub;
use crate::{CONTROLLER_NAMESPACE, JUPSOL_MINT_PUBKEY};

pub const KAMINO_DEPOSITORY_SEED: &[u8] = b"kamino_depository";

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
        seeds = [KAMINO_DEPOSITORY_SEED],
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

impl<'info> Redeem<'info> {
    fn burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            from: self.user_redeemable.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.kamino_depository.to_account_info(),
            to: self.user_collateral.to_account_info(),
            authority: self.kamino_depository.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
    // 驗證可贖回金額
    require!(redeemable_amount.value() > 0, XxusdError::InvalidRedeemableAmount);
    require!(
        ctx.accounts.user_redeemable.amount >= redeemable_amount.value(),
        XxusdError::InsufficientRedeemableBalance
    );

    // 銷毀 xxUSD
    token::burn(ctx.accounts.burn_context(), redeemable_amount.value())?;

    // 從 Kamino 存儲庫轉移 jupSOL 到用戶
    let collateral_amount = redeemable_amount; // 假設 1:1 兌換，實際情況可能需要更複雜的計算
    let seeds = &[
        KAMINO_DEPOSITORY_SEED.as_ref(),
        &[ctx.bumps.kamino_depository],
    ];
    let signer = &[&seeds[..]];
    token::transfer(
        ctx.accounts.transfer_context().with_signer(signer),
        collateral_amount.value()
    )?;

    // 更新狀態
    ctx.accounts.controller.reload()?;
    ctx.accounts.kamino_depository.reload()?;

    let controller = &mut ctx.accounts.controller;
    let current_supply = Amount::from_u128(controller.get_redeemable_circulating_supply())?;
    let new_supply = checked_sub(current_supply, redeemable_amount)?;
    controller.set_redeemable_circulating_supply(new_supply.to_u128())?;

    let kamino_depository = &mut ctx.accounts.kamino_depository;
    let current_amount_under_management = Amount::from_u128(kamino_depository.redeemable_amount_under_management)?;
    let new_amount_under_management = checked_sub(current_amount_under_management, redeemable_amount)?;
    kamino_depository.redeemable_amount_under_management = new_amount_under_management.to_u128();

    // 發出事件
    emit!(RedeemEvent {
        user: *ctx.accounts.user.key,
        redeemable_amount,
        collateral_amount,
    });

    Ok(())
}

#[event]
pub struct RedeemEvent {
    pub user: Pubkey,
    pub redeemable_amount: Amount,
    pub collateral_amount: Amount,
}