use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::XxusdError;
use crate::state::{controller::Controller, lock_manager::LockManager};
use crate::core::{Amount, Timestamp, i64_to_timestamp, timestamp_to_i64};
use crate::utils::maths::{checked_sub, checked_sub_timestamp};

pub const CONTROLLER_SEED: &[u8] = b"controller";
pub const LOCK_MANAGER_SEED: &[u8] = b"lock_manager";

#[derive(Accounts)]
pub struct ReleaseXxusd<'info> {
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

impl<'info> ReleaseXxusd<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.lock_vault.to_account_info(),
            to: self.user_xxusd.to_account_info(),
            authority: self.lock_manager.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<ReleaseXxusd>) -> Result<()> {
    let current_time = i64_to_timestamp(Clock::get()?.unix_timestamp);

    // 執行不可變操作
    let (releasable_amount, current_total_locked_amount, current_locked_supply) = 
        perform_immutable_operations(&ctx.accounts, current_time)?;

    // 執行可變操作
    perform_mutable_operations(ctx, current_time, releasable_amount, current_total_locked_amount, current_locked_supply)?;

    // 發出釋放事件
    emit!(ReleaseEvent {
        user: *ctx.accounts.user.key,
        amount: releasable_amount,
    });

    Ok(())
}

fn perform_immutable_operations(accounts: &ReleaseXxusd, current_time: Timestamp) -> Result<(Amount, Amount, u128)> {
    let lock_period = timestamp_to_i64(accounts.lock_manager.locks[0].lock_period) as u64;
    require!(lock_period > 0, XxusdError::InvalidLockPeriod);

    let current_total_locked_amount = accounts.lock_manager.get_total_locked_amount();
    let current_locked_supply = accounts.controller.get_locked_xxusd_supply();

    // 計算可釋放金額
    let (releasable_amount, _) = calculate_releasable_amount(&accounts.lock_manager, current_time)?;

    Ok((releasable_amount, current_total_locked_amount, current_locked_supply))
}

fn perform_mutable_operations(
    ctx: Context<ReleaseXxusd>,
    current_time: Timestamp,
    releasable_amount: Amount,
    current_total_locked_amount: Amount,
    current_locked_supply: u128,
) -> Result<()> {
    let lock_manager = &mut ctx.accounts.lock_manager;
    let user_lock = lock_manager.locks.iter_mut()
        .find(|lock| lock.amount.value() > 0)
        .ok_or(XxusdError::LockNotFound)?;

    // 更新鎖定狀態
    user_lock.amount = checked_sub(user_lock.amount, releasable_amount)?;
    user_lock.lock_time = current_time;

    // 轉移 xxUSD 從鎖定保管庫到用戶
    let seeds = &[
        LOCK_MANAGER_SEED.as_ref(),
        &[ctx.bumps.lock_manager],
    ];
    let signer = &[&seeds[..]];
    token::transfer(
        ctx.accounts.transfer_context().with_signer(signer),
        releasable_amount.value()
    )?;

    // 更新鎖定管理器狀態
    let new_total_locked_amount = checked_sub(current_total_locked_amount, releasable_amount)?;
    lock_manager.set_total_locked_amount(new_total_locked_amount);

    // 更新控制器狀態
    let controller = &mut ctx.accounts.controller;
    let new_locked_supply = checked_sub(Amount::from_u128(current_locked_supply)?, releasable_amount)?;
    controller.set_locked_xxusd_supply(new_locked_supply.to_u128())?;

    Ok(())
}

fn calculate_releasable_amount(lock_manager: &LockManager, current_time: Timestamp) -> Result<(Amount, u64)> {
    let user_lock = lock_manager.locks.iter()
        .find(|lock| lock.amount.value() > 0)
        .ok_or(XxusdError::LockNotFound)?;

    let days_passed = checked_sub_timestamp(current_time, user_lock.lock_time)?;
    let days_passed_u64 = timestamp_to_i64(Timestamp(days_passed)) as u64 / 86400; // 86400 seconds in a day
    let lock_period = timestamp_to_i64(user_lock.lock_period) as u64;

    let releasable_amount = Amount::from_u128(std::cmp::min(
        user_lock.amount.to_u128(),
        days_passed_u64 as u128 * user_lock.amount.to_u128() / lock_period as u128
    )).map_err(|_| XxusdError::Overflow)?;

    require!(releasable_amount.value() > 0, XxusdError::InsufficientReleasableAmount);

    Ok((releasable_amount, days_passed_u64))
}

#[event]
pub struct ReleaseEvent {
    pub user: Pubkey,
    pub amount: Amount,
}