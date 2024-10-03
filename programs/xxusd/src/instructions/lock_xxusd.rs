use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, lock_manager::LockManager, Amount, Timestamp};
use crate::utils::maths::checked_add;

pub const CONTROLLER_SEED: &[u8] = b"controller";
pub const LOCK_MANAGER_SEED: &[u8] = b"lock_manager";

#[derive(Accounts)]
pub struct LockXxusd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [CONTROLLER_SEED],
        bump,
    )]
    pub controller: Box<Account<'info, Controller>>,

    #[account(
        mut,
        seeds = [LOCK_MANAGER_SEED],
        bump,
    )]
    pub lock_manager: Box<Account<'info, LockManager>>,

    #[account(
        mut,
        constraint = user_xxusd.owner == user.key() @XxusdError::InvalidOwner,
        constraint = user_xxusd.mint == controller.xxusd_mint @XxusdError::InvalidMint,
    )]
    pub user_xxusd: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = lock_vault.owner == lock_manager.key() @XxusdError::InvalidOwner,
        constraint = lock_vault.mint == controller.xxusd_mint @XxusdError::InvalidMint,
    )]
    pub lock_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> LockXxusd<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_xxusd.to_account_info(),
            to: self.lock_vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
    // Validate lock amount and period
    require!(amount.value() > 0, XxusdError::InvalidCollateralAmount);
    require!(lock_period.value() > 0, XxusdError::InvalidLockPeriod);
    require!(ctx.accounts.user_xxusd.amount >= amount.value(), XxusdError::InsufficientBalance);

    // Transfer xxUSD from user to lock vault
    token::transfer(ctx.accounts.transfer_context(), amount.value())?;

    // Update lock manager state
    ctx.accounts.lock_manager.reload()?;
    let lock_manager = &mut ctx.accounts.lock_manager;
    let current_locked_amount = lock_manager.get_total_locked_amount();
    let new_total_locked_amount = checked_add(current_locked_amount, amount)?;
    lock_manager.set_total_locked_amount(new_total_locked_amount);
    lock_manager.locks.push(crate::state::lock_manager::Lock {
        amount,
        lock_time: Timestamp::new(Clock::get()?.unix_timestamp),
        lock_period,
    });

    // Update controller state
    ctx.accounts.controller.reload()?;
    let controller = &mut ctx.accounts.controller;
    let current_locked_supply = Amount::from_u128(controller.get_locked_xxusd_supply())?;
    let new_locked_supply = checked_add(current_locked_supply, amount)?;
    controller.set_locked_xxusd_supply(new_locked_supply.to_u128())?;

    // Emit lock event
    emit!(LockEvent {
        user: *ctx.accounts.user.key,
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