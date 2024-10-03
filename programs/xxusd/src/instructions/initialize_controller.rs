use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::Controller;
use crate::error::XxusdError;

pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const DEFAULT_MAX_PRODUCTS: u64 = 100; // 設置一個默認值，您可以根據需要調整

#[derive(Accounts)]
#[instruction(redeemable_mint_decimals: u8)]
pub struct InitializeController<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Controller::LEN,
        seeds = [CONTROLLER_NAMESPACE],
        bump
    )]
    pub controller: Box<Account<'info, Controller>>,

    #[account(
        init,
        payer = authority,
        mint::decimals = redeemable_mint_decimals,
        mint::authority = controller,
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,

    pub xxusd_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeController>,
    redeemable_mint_decimals: u8,
) -> Result<()> {
    if redeemable_mint_decimals > 9 {
        return Err(XxusdError::InvalidRedeemableMintDecimals.into());
    }

    let controller = &mut ctx.accounts.controller;
    controller.initialize(
        ctx.bumps.controller,
        ctx.accounts.authority.key(),
        ctx.accounts.redeemable_mint.key(),
        ctx.accounts.xxusd_mint.key(),
        DEFAULT_MAX_PRODUCTS,
    )?;

    // Note: The following default values are set during initialization:
    // - redeemable_circulating_supply: Amount::new(0)
    // - kamino_depository: Pubkey::default()
    // - kamino_depository_weight_bps: 10000 (100%)
    // - is_frozen: false
    // - product_prices: empty Vec
    // - locked_xxusd_supply: Amount::new(0)
    // - max_products: DEFAULT_MAX_PRODUCTS

    Ok(())
}