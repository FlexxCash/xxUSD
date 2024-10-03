use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, lock_manager::LockManager};
use crate::core::{Amount, Timestamp, i64_to_timestamp};
use crate::utils::maths::checked_add;

#[derive(Accounts)]
pub struct LockXxusd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"controller"],
        bump,
    )]
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [b"lock_manager"],
        bump,
    )]
    pub lock_manager: Account<'info, LockManager>,

    #[account(
        mut,
        constraint = user_xxusd.owner == user.key() @XxusdError::InvalidOwner,
        constraint = user_xxusd.mint == controller.xxusd_mint @XxusdError::InvalidMint,
    )]
    pub user_xxusd: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lock_vault.owner == lock_manager.key() @XxusdError::InvalidOwner,
        constraint = lock_vault.mint == controller.xxusd_mint @XxusdError::InvalidMint,
    )]
    pub lock_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
    let user = &ctx.accounts.user;
    let controller = &mut ctx.accounts.controller;
    let lock_manager = &mut ctx.accounts.lock_manager;
    let user_xxusd = &mut ctx.accounts.user_xxusd;
    let lock_vault = &mut ctx.accounts.lock_vault;

    // Validate lock period
    require!(lock_period.value() > 0, XxusdError::InvalidLockPeriod);

    // Transfer xxUSD from user to lock vault
    let cpi_accounts = Transfer {
        from: user_xxusd.to_account_info(),
        to: lock_vault.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount.value())?;

    // Update lock manager state
    let current_locked_amount = lock_manager.get_total_locked_amount();
    let new_total_locked_amount = checked_add(current_locked_amount, amount)?;
    lock_manager.set_total_locked_amount(new_total_locked_amount);
    lock_manager.locks.push(crate::state::lock_manager::Lock {
        amount,
        lock_time: i64_to_timestamp(Clock::get()?.unix_timestamp),
        lock_period,
    });

    // Update controller state
    let current_locked_supply = Amount::from_u128(controller.get_locked_xxusd_supply())?;
    let new_locked_supply = checked_add(current_locked_supply, amount)?;
    controller.set_locked_xxusd_supply(new_locked_supply.to_u128())?;

    // Emit lock event
    emit!(LockEvent {
        user: *user.key,
        amount,
        lock_period,
    });

    Ok(())
}

#[event]
pub struct LockEvent {
    pub user: Pubkey,
    pub amount: Amount,
    pub lock_period: Timestamp,
}