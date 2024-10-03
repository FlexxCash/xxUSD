use anchor_lang::prelude::*;
use crate::core::{Amount, u64_to_amount};

#[account]
pub struct HedgingStrategy {
    pub bump: u8,
    pub controller: Pubkey,
    pub deposited_amount: u64,
    // ... 其他字段 ...
}

impl HedgingStrategy {
    pub const LEN: usize = 8 + std::mem::size_of::<HedgingStrategy>();

    pub fn initialize(&mut self, bump: u8, controller: Pubkey) -> Result<()> {
        self.bump = bump;
        self.controller = controller;
        self.deposited_amount = 0;
        Ok(())
    }

    pub fn get_deposited_amount(&self) -> Amount {
        u64_to_amount(self.deposited_amount)
    }

    pub fn set_deposited_amount(&mut self, amount: Amount) {
        self.deposited_amount = amount.value();
    }

    // ... 其他方法 ...
}