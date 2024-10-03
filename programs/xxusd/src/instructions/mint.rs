use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::error::XxusdError;
use crate::state::{Controller, KaminoDepository};
use crate::{CONTROLLER_NAMESPACE, JUPSOL_MINT_PUBKEY};
use crate::core::{Amount, u64_to_amount, safe_u128_to_u64};
use crate::utils::maths::checked_add;

#[derive(Accounts)]
pub struct MintInstruction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_NAMESPACE],
        bump = controller.bump,
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
        bump = kamino_depository.bump,
        has_one = controller @XxusdError::InvalidController,
        has_one = collateral_mint @XxusdError::InvalidCollateralMint,
    )]
    pub kamino_depository: Box<Account<'info, KaminoDepository>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
    // 註解：這個函數處理 xxUSD 的鑄造邏輯
    // 在 UXD 中，這裡使用了 credix_client 和 xxusd_cpi 來處理不同的存儲庫
    // 在我們的 xxUSD 實現中，我們將直接處理 Kamino 存儲庫

    // 1. 驗證抵押品金額
    if collateral_amount.value() == 0 {
        return Err(XxusdError::InvalidCollateralAmount.into());
    }

    // 2. 計算要鑄造的 xxUSD 數量（這裡假設 1:1 兌換，實際情況可能需要更複雜的計算）
    let xxusd_amount = collateral_amount;

    // 3. 從用戶轉移 jupSOL 到 Kamino 存儲庫
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_collateral.to_account_info(),
                to: ctx.accounts.kamino_depository.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        xxusd_amount.value(),
    )?;

    // 4. 鑄造 xxUSD
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.redeemable_mint.to_account_info(),
                to: ctx.accounts.user_redeemable.to_account_info(),
                authority: ctx.accounts.controller.to_account_info(),
            },
            &[&[
                CONTROLLER_NAMESPACE,
                &[ctx.accounts.controller.bump],
            ]],
        ),
        xxusd_amount.value(),
    )?;

    // 5. 更新狀態
    let controller = &mut ctx.accounts.controller;
    let current_supply = Amount::from_u128(controller.get_redeemable_circulating_supply())?;
    let new_supply = checked_add(current_supply, xxusd_amount)?;
    controller.set_redeemable_circulating_supply(new_supply.to_u128())?;

    let kamino_depository = &mut ctx.accounts.kamino_depository;
    let current_amount_under_management = u64_to_amount(kamino_depository.redeemable_amount_under_management.try_into().unwrap());
    let new_amount_under_management = checked_add(current_amount_under_management, xxusd_amount)?;
    kamino_depository.redeemable_amount_under_management = safe_u128_to_u64(new_amount_under_management.to_u128())?.into();

    Ok(())
}