pub mod initialize_controller;
pub mod mint;
pub mod redeem;
pub mod lock_xxusd;
pub mod release_xxusd;
pub mod manage_product_price;
pub mod manage_hedging_strategy;
pub mod freeze_program;
pub mod edit_controller;

use anchor_lang::prelude::*;
use crate::core::{Amount, Timestamp};

pub use initialize_controller::*;
pub use mint::*;
pub use redeem::*;
pub use lock_xxusd::*;
pub use release_xxusd::*;
pub use manage_product_price::*;
pub use manage_hedging_strategy::*;
pub use freeze_program::*;
pub use edit_controller::*;

pub fn initialize_controller(ctx: Context<InitializeController>, redeemable_mint_decimals: u8) -> Result<()> {
    initialize_controller::handler(ctx, redeemable_mint_decimals)
}

pub fn mint(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
    mint::handler(ctx, collateral_amount)
}

pub fn redeem(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
    redeem::handler(ctx, redeemable_amount)
}

pub fn lock_xxusd(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
    lock_xxusd::handler(ctx, amount, lock_period)
}

pub fn release_xxusd(ctx: Context<ReleaseXxusd>) -> Result<()> {
    release_xxusd::handler(ctx)
}

pub fn manage_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
    manage_product_price::handler(ctx, product_id, price)
}

pub fn manage_hedging_strategy(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
    manage_hedging_strategy::handler(ctx, amount, is_deposit)
}

pub fn freeze_program(ctx: Context<FreezeProgram>, freeze: bool) -> Result<()> {
    freeze_program::handler(ctx, freeze)
}

pub fn edit_controller(ctx: Context<EditController>, new_authority: Option<Pubkey>) -> Result<()> {
    edit_controller::handler(ctx, new_authority)
}