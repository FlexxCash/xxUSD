use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer, MintTo};
use anchor_spl::associated_token::AssociatedToken;

use crate::error::XxusdError;
use crate::state::{Controller, KaminoDepository, Amount};
use crate::{CONTROLLER_NAMESPACE, JUPSOL_MINT_PUBKEY};
use crate::utils::maths::checked_add;

pub const KAMINO_DEPOSITORY_SEED: &[u8] = b"kamino_depository";

#[derive(Accounts)]
pub struct MintInstruction<'info> {
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

impl<'info> MintInstruction<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_collateral.to_account_info(),
            to: self.kamino_depository.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.user_redeemable.to_account_info(),
            authority: self.controller.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
    // 1. 驗證抵押品金額
    require!(collateral_amount.value() > 0, XxusdError::InvalidCollateralAmount);

    // 2. 計算要鑄造的 xxUSD 數量（這裡假設 1:1 兌換，實際情況可能需要更複雜的計算）
    let xxusd_amount = collateral_amount;

    // 3. 檢查用戶是否有足夠的抵押品
    require!(
        ctx.accounts.user_collateral.amount >= collateral_amount.value(),
        XxusdError::InsufficientCollateral
    );

    // 4. 從用戶轉移 jupSOL 到 Kamino 存儲庫
    token::transfer(ctx.accounts.transfer_context(), xxusd_amount.value())?;

    // 5. 鑄造 xxUSD
    let seeds = &[
        CONTROLLER_NAMESPACE.as_ref(),
        &[ctx.bumps.controller],
    ];
    let signer = &[&seeds[..]];
    token::mint_to(ctx.accounts.mint_context().with_signer(signer), xxusd_amount.value())?;

    // 6. 更新狀態
    ctx.accounts.controller.reload()?;
    ctx.accounts.kamino_depository.reload()?;

    let controller = &mut ctx.accounts.controller;
    let current_supply = Amount::from_u128(controller.get_redeemable_circulating_supply())?;
    let new_supply = checked_add(current_supply, xxusd_amount)?;
    controller.set_redeemable_circulating_supply(new_supply.to_u128())?;

    let kamino_depository = &mut ctx.accounts.kamino_depository;
    let current_amount_under_management = Amount::from_u128(kamino_depository.redeemable_amount_under_management)?;
    let new_amount_under_management = checked_add(current_amount_under_management, xxusd_amount)?;
    kamino_depository.redeemable_amount_under_management = new_amount_under_management.to_u128();

    // 7. 發出事件
    emit!(MintEvent {
        user: *ctx.accounts.user.key,
        collateral_amount,
        xxusd_amount,
    });

    Ok(())
}

#[event]
pub struct MintEvent {
    pub user: Pubkey,
    pub collateral_amount: Amount,
    pub xxusd_amount: Amount,
}