use anchor_lang::prelude::*;
use crate::state::{Amount, Timestamp};
use crate::state::u64_to_amount;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct Lock {
    pub amount: Amount,
    pub lock_time: Timestamp,
    pub lock_period: Timestamp,
}

#[account]
pub struct LockManager {
    pub bump: u8,
    pub controller: Pubkey,
    pub total_locked_amount: u64,
    pub locks: Vec<Lock>,
}

impl LockManager {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 4 + (32 * 10); // 假設最多存儲10個鎖定記錄

    pub fn initialize(&mut self, bump: u8, controller: Pubkey) -> anchor_lang::Result<()> {
        self.bump = bump;
        self.controller = controller;
        self.total_locked_amount = 0;
        self.locks = Vec::new();
        Ok(())
    }

    pub fn get_total_locked_amount(&self) -> Amount {
        u64_to_amount(self.total_locked_amount)
    }

    pub fn set_total_locked_amount(&mut self, amount: Amount) {
        self.total_locked_amount = amount.value();
    }

    // 添加其他必要的方法
}