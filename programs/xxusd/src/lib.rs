use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;
pub mod error;
pub mod events;

use instructions::*;
use state::*;
use error::XxusdError;

use state::{Amount, Timestamp};

// 定義常量
pub const CONTROLLER_NAMESPACE: &[u8] = b"controller";
pub const JUPSOL_MINT_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]); // 替換為實際的 JUPSOL_MINT_PUBKEY

declare_id!("Cpsquy1RbEb4N3FXDKBzrWMKTLLvBp1BBSvp899EHhCb");

#[program]
pub mod xxusd {
    use super::*;

    pub fn initialize_controller(ctx: Context<InitializeController>, params: InitializeControllerParams) -> anchor_lang::Result<()> {
        initialize_controller_handler(ctx, params)
    }

    pub fn lock_xxusd(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> anchor_lang::Result<()> {
        lock_xxusd_handler(ctx, amount, lock_period)
    }

    pub fn release_xxusd(ctx: Context<ReleaseXxusd>) -> anchor_lang::Result<()> {
        release_xxusd_handler(ctx)
    }

    pub fn mint(ctx: Context<MintInstruction>, collateral_amount: Amount) -> anchor_lang::Result<()> {
        mint_handler(ctx, collateral_amount)
    }

    pub fn redeem(ctx: Context<Redeem>, redeemable_amount: Amount) -> anchor_lang::Result<()> {
        redeem_handler(ctx, redeemable_amount)
    }

    pub fn manage_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> anchor_lang::Result<()> {
        manage_product_price_handler(ctx, product_id, price)
    }

    pub fn manage_hedging_strategy(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> anchor_lang::Result<()> {
        manage_hedging_strategy_handler(ctx, amount, is_deposit)
    }
}