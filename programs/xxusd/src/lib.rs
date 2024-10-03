use std::cell::Ref;

use crate::instructions::*;
use crate::state::*;
use anchor_lang::prelude::*;
use error::XxusdError;

#[macro_use]
pub mod instructions;
pub mod oracle;
pub mod state;
pub mod utils;
pub mod core;
pub mod error;
pub mod events;

use state::{Amount, Timestamp};

use instructions::initialize_controller::{InitializeController, InitializeControllerParams};
use instructions::lock_xxusd::LockXxusd;
use instructions::release_xxusd::ReleaseXxusd;
use instructions::mint::MintInstruction;
use instructions::redeem::Redeem;
use instructions::manage_product_price::ManageProductPrice;
use instructions::manage_hedging_strategy::ManageHedgingStrategy;

// 定義常量
pub const CONTROLLER_NAMESPACE: &[u8] = b"controller";
pub const JUPSOL_MINT_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]); // 替換為實際的 JUPSOL_MINT_PUBKEY

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod xxusd {
    use super::*;

    pub fn initialize_controller(ctx: Context<InitializeController>, params: InitializeControllerParams) -> Result<()> {
        instructions::initialize_controller::handler(ctx, params)
    }

    pub fn lock_xxusd(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
        instructions::lock_xxusd::handler(ctx, amount, lock_period)
    }

    pub fn release_xxusd(ctx: Context<ReleaseXxusd>) -> Result<()> {
        instructions::release_xxusd::handler(ctx)
    }

    pub fn mint(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
        instructions::mint::handler(ctx, collateral_amount)
    }

    pub fn redeem(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
        instructions::redeem::handler(ctx, redeemable_amount)
    }

    pub fn manage_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
        instructions::manage_product_price::handler(ctx, product_id, price)
    }

    pub fn manage_hedging_strategy(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
        instructions::manage_hedging_strategy::handler(ctx, amount, is_deposit)
    }
}