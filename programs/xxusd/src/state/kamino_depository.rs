use anchor_lang::prelude::*;

#[account]
pub struct KaminoDepository {
    pub bump: u8,
    pub controller: Pubkey,
    pub collateral_mint: Pubkey,
    pub redeemable_amount_under_management: u128,
    pub redeemable_amount_under_management_cap: u128,
    pub minting_fee_in_bps: u16,
    pub redeeming_fee_in_bps: u16,
    // 添加其他必要的字段...
}

impl KaminoDepository {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 16 + 16 + 2 + 2;

    pub fn initialize(
        &mut self,
        bump: u8,
        controller: Pubkey,
        collateral_mint: Pubkey,
        redeemable_amount_under_management_cap: u128,
        minting_fee_in_bps: u16,
        redeeming_fee_in_bps: u16,
    ) -> Result<()> {
        self.bump = bump;
        self.controller = controller;
        self.collateral_mint = collateral_mint;
        self.redeemable_amount_under_management = 0;
        self.redeemable_amount_under_management_cap = redeemable_amount_under_management_cap;
        self.minting_fee_in_bps = minting_fee_in_bps;
        self.redeeming_fee_in_bps = redeeming_fee_in_bps;
        Ok(())
    }

    // 添加其他必要的方法...
}
