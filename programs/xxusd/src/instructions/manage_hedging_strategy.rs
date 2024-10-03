use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, hedging_strategy::HedgingStrategy};
use crate::core::Amount;
use crate::utils::maths::{checked_add, checked_sub};

#[derive(Accounts)]
pub struct ManageHedgingStrategy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"controller"],
        bump,
        has_one = authority,
    )]
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [b"hedging_strategy"],
        bump,
    )]
    pub hedging_strategy: Account<'info, HedgingStrategy>,

    #[account(mut)]
    pub source_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
    if is_deposit {
        deposit_to_lending_platform(ctx, amount)
    } else {
        withdraw_from_lending_platform(ctx, amount)
    }
}

pub fn deposit_to_lending_platform(ctx: Context<ManageHedgingStrategy>, amount: Amount) -> Result<()> {
    let hedging_strategy = &mut ctx.accounts.hedging_strategy;

    // Transfer tokens to lending platform
    let cpi_accounts = Transfer {
        from: ctx.accounts.source_account.to_account_info(),
        to: ctx.accounts.destination_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount.value())?;

    // Update hedging strategy state
    let new_deposited_amount = checked_add(
        hedging_strategy.get_deposited_amount(),
        amount
    )?;
    hedging_strategy.set_deposited_amount(new_deposited_amount);

    Ok(())
}

pub fn withdraw_from_lending_platform(ctx: Context<ManageHedgingStrategy>, amount: Amount) -> Result<()> {
    let hedging_strategy = &mut ctx.accounts.hedging_strategy;

    // Ensure we have enough deposited to withdraw
    require!(hedging_strategy.get_deposited_amount().value() >= amount.value(), XxusdError::InsufficientFunds);

    // Transfer tokens from lending platform
    let cpi_accounts = Transfer {
        from: ctx.accounts.source_account.to_account_info(),
        to: ctx.accounts.destination_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount.value())?;

    // Update hedging strategy state
    let new_deposited_amount = checked_sub(
        hedging_strategy.get_deposited_amount(),
        amount
    )?;
    hedging_strategy.set_deposited_amount(new_deposited_amount);

    Ok(())
}

pub fn swap_assets(ctx: Context<ManageHedgingStrategy>, amount_in: Amount, min_amount_out: Amount) -> Result<()> {
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
pub struct SwapEvent {
    pub amount_in: Amount,
    pub min_amount_out: Amount,
}