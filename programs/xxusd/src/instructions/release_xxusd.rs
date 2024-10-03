use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, lock_manager::LockManager};
use crate::core::{Amount, Timestamp, i64_to_timestamp, timestamp_to_i64, safe_u128_to_u64};
use crate::utils::maths::{checked_sub, checked_sub_timestamp};

#[derive(Accounts)]
pub struct ReleaseXxusd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"controller"],
        bump = controller.bump,
    )]
    pub controller: Account<'info, Controller>,

    #[account(
        mut,
        seeds = [b"lock_manager"],
        bump = lock_manager.bump,
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

                    pub fn handler(ctx: Context<ReleaseXxusd>) -> Result<()> {
                        let user = &ctx.accounts.user;
                        let controller = &mut ctx.accounts.controller;
                        let lock_manager = &mut ctx.accounts.lock_manager;
                        let user_xxusd = &mut ctx.accounts.user_xxusd;
                        let lock_vault = &mut ctx.accounts.lock_vault;

                        let current_time = i64_to_timestamp(Clock::get()?.unix_timestamp);

                        // Find the user's lock
                        let user_lock = lock_manager.locks.iter_mut()
                            .find(|lock| lock.amount.value() > 0)
                            .ok_or(XxusdError::LockNotFound)?;

                        // Calculate releasable amount
                        let days_passed = checked_sub_timestamp(current_time, user_lock.lock_time)?;
                        let days_passed_u64 = timestamp_to_i64(Timestamp(days_passed)) as u64 / 86400; // 86400 seconds in a day
                        let lock_period = timestamp_to_i64(user_lock.lock_period) as u64;
                        let releasable_amount = Amount::from_u128(std::cmp::min(
                            user_lock.amount.to_u128(),
                            (days_passed_u64 as u128 * user_lock.amount.to_u128() / lock_period as u128)
                        )).map_err(|_| XxusdError::Overflow)?;

                        // Update lock state
                        user_lock.amount = checked_sub(user_lock.amount, releasable_amount)?;
                        user_lock.lock_time = current_time;

                        // Transfer xxUSD from lock vault to user
                        let seeds = &[
                            b"lock_manager".as_ref(),
                            &[lock_manager.bump],
                        ];
                        let signer = &[&seeds[..]];

                        let cpi_accounts = Transfer {
                            from: lock_vault.to_account_info(),
                            to: user_xxusd.to_account_info(),
                            authority: lock_manager.to_account_info(),
                        };
                        let cpi_program = ctx.accounts.token_program.to_account_info();
                        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                        token::transfer(cpi_ctx, releasable_amount.value())?;

                        // Update lock manager state
                        let current_total_locked_amount = lock_manager.get_total_locked_amount();
                        let new_total_locked_amount = checked_sub(current_total_locked_amount, releasable_amount)?;
                        lock_manager.set_total_locked_amount(new_total_locked_amount);

                        // Update controller state
                        let current_locked_supply = controller.get_locked_xxusd_supply();
                        let new_locked_supply = checked_sub(Amount::from_u128(current_locked_supply)?, releasable_amount)?;
                        controller.set_locked_xxusd_supply(new_locked_supply.to_u128())?;

                        // Emit release event
                        emit!(ReleaseEvent {
                            user: *user.key,
                            amount: releasable_amount,
                        });

                        Ok(())
                    }

#[event]
pub struct ReleaseEvent {
    pub user: Pubkey,
    pub amount: Amount,
}