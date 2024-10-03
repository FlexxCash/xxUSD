use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, hedging_strategy::HedgingStrategy};
use crate::core::Amount;
use crate::utils::maths::{checked_add, checked_sub};

pub const CONTROLLER_SEED: &[u8] = b"controller";
pub const HEDGING_STRATEGY_SEED: &[u8] = b"hedging_strategy";

#[derive(Accounts)]
pub struct ManageHedgingStrategy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_SEED],
        bump,
        has_one = authority,
    )]
    pub controller: Box<Account<'info, Controller>>,

    #[account(
        mut,
        seeds = [HEDGING_STRATEGY_SEED],
        bump,
    )]
    pub hedging_strategy: Box<Account<'info, HedgingStrategy>>,

    #[account(mut)]
    pub source_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ManageHedgingStrategy<'info> {
    fn transfer_context(&self, from: &AccountInfo<'info>, to: &AccountInfo<'info>, authority: &AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: from.clone(),
            to: to.clone(),
            authority: authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
    require!(amount.value() > 0, XxusdError::InvalidAmount);

    if is_deposit {
        deposit_to_lending_platform(ctx, amount)
    } else {
        withdraw_from_lending_platform(ctx, amount)
    }
}

fn deposit_to_lending_platform(ctx: Context<ManageHedgingStrategy>, amount: Amount) -> Result<()> {
    // Check if source account has enough balance
    require!(
        ctx.accounts.source_account.amount >= amount.value(),
        XxusdError::InsufficientFunds
    );

    // Get the current deposited amount
    let current_deposited_amount = ctx.accounts.hedging_strategy.get_deposited_amount();

    // Transfer tokens to lending platform
    token::transfer(
        ctx.accounts.transfer_context(
            &ctx.accounts.source_account.to_account_info(),
            &ctx.accounts.destination_account.to_account_info(),
            &ctx.accounts.authority.to_account_info()
        ),
        amount.value()
    )?;

    // Update hedging strategy state
    let new_deposited_amount = checked_add(current_deposited_amount, amount)?;
    ctx.accounts.hedging_strategy.set_deposited_amount(new_deposited_amount);

    // Emit deposit event
    emit!(DepositEvent {
        amount,
        new_total_deposited: new_deposited_amount,
    });

    Ok(())
}

fn withdraw_from_lending_platform(ctx: Context<ManageHedgingStrategy>, amount: Amount) -> Result<()> {
    // Get the current deposited amount
    let current_deposited_amount = ctx.accounts.hedging_strategy.get_deposited_amount();

    // Ensure we have enough deposited to withdraw
    require!(
        current_deposited_amount.value() >= amount.value(),
        XxusdError::InsufficientFunds
    );

    // Transfer tokens from lending platform
    token::transfer(
        ctx.accounts.transfer_context(
            &ctx.accounts.source_account.to_account_info(),
            &ctx.accounts.destination_account.to_account_info(),
            &ctx.accounts.authority.to_account_info()
        ),
        amount.value()
    )?;

    // Update hedging strategy state
    let new_deposited_amount = checked_sub(current_deposited_amount, amount)?;
    ctx.accounts.hedging_strategy.set_deposited_amount(new_deposited_amount);

    // Emit withdraw event
    emit!(WithdrawEvent {
        amount,
        new_total_deposited: new_deposited_amount,
    });

    Ok(())
}

pub fn swap_assets(_ctx: Context<ManageHedgingStrategy>, amount_in: Amount, min_amount_out: Amount) -> Result<()> {
    // Here you would implement the logic to swap assets
    // This might involve calling an external DEX or AMM

    // For now, we'll just emit an event
    emit!(SwapEvent {
        amount_in,
        min_amount_out,
    });

    Ok(())
}

#[event]
pub struct DepositEvent {
    pub amount: Amount,
    pub new_total_deposited: Amount,
}

#[event]
pub struct WithdrawEvent {
    pub amount: Amount,
    pub new_total_deposited: Amount,
}

#[event]
pub struct SwapEvent {
    pub amount_in: Amount,
    pub min_amount_out: Amount,
}