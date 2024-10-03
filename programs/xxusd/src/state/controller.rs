use anchor_lang::prelude::*;
use crate::core::{Amount, u64_to_amount, safe_u128_to_u64, safe_u64_to_u128};
use crate::error::XxusdError;

#[account]
pub struct Controller {
    pub bump: u8,
    pub authority: Pubkey,
    pub redeemable_mint: Pubkey,
    pub xxusd_mint: Pubkey,
    pub redeemable_circulating_supply: u64,
    pub kamino_depository: Pubkey,
    pub kamino_depository_weight_bps: u16,
    pub is_frozen: bool,
    pub product_prices: Vec<(u64, Amount)>,
    pub locked_xxusd_supply: u64,
}

impl Controller {
    pub const LEN: usize = 8 + std::mem::size_of::<Controller>();
    
    pub fn initialize(
        &mut self,
        bump: u8,
        authority: Pubkey,
        redeemable_mint: Pubkey,
        xxusd_mint: Pubkey,
    ) -> Result<()> {
        self.bump = bump;
        self.authority = authority;
        self.redeemable_mint = redeemable_mint;
        self.xxusd_mint = xxusd_mint;
        self.redeemable_circulating_supply = 0;
        self.kamino_depository = Pubkey::default();
        self.kamino_depository_weight_bps = 10000; // 100%
        self.is_frozen = false;
        self.product_prices = Vec::new();
        self.locked_xxusd_supply = 0;
        Ok(())
    }

    pub fn get_redeemable_circulating_supply(&self) -> u128 {
        safe_u64_to_u128(self.redeemable_circulating_supply)
    }

    pub fn set_redeemable_circulating_supply(&mut self, amount: u128) -> Result<()> {
        self.redeemable_circulating_supply = safe_u128_to_u64(amount)?;
        Ok(())
    }

    pub fn get_locked_xxusd_supply(&self) -> u128 {
        safe_u64_to_u128(self.locked_xxusd_supply)
    }

    pub fn set_locked_xxusd_supply(&mut self, amount: u128) -> Result<()> {
        self.locked_xxusd_supply = safe_u128_to_u64(amount)?;
        Ok(())
    }

    pub fn load_mut(&mut self) -> Result<()> {
        // 實現 load_mut 方法
        Ok(())
    }
}