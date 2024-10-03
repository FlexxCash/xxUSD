use anchor_lang::prelude::*;
use std::panic;

pub mod core;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod oracle;

pub use state::controller::Controller;
pub use core::{Amount, Timestamp, NumericType};
pub const JUPSOL_MINT_PUBKEY: Pubkey = solana_program::pubkey!("7eS55f4LP5xj4jqRp24uv5aPFak4gzue8jwb5949KDzP");
pub const USDC_MINT_PUBKEY: Pubkey = solana_program::pubkey!("EneKhgmdLQgfLtqC9aE52B1bMcFtjob6qMkDc5Q3mHx7");

pub const CONTROLLER_NAMESPACE: &[u8] = b"CONTROLLER";
pub const BPS_POWER: u64 = 10000;
pub const MAX_REDEEMABLE_GLOBAL_SUPPLY_CAP: u128 = u128::MAX;

declare_id!("8ge5JzwZSYo8A3Qcrt2x9Zbug4umokMtZC6tMNBoFU9Z");

#[program]
pub mod xxusd {
    use super::*;

    pub fn initialize_controller(ctx: Context<instructions::InitializeController>, redeemable_mint_decimals: u8) -> Result<()> {
        instructions::initialize_controller::handler(ctx, redeemable_mint_decimals)
    }

    pub fn mint(ctx: Context<instructions::MintInstruction>, collateral_amount: Amount) -> Result<()> {
        instructions::mint::handler(ctx, collateral_amount)
    }

    pub fn redeem(ctx: Context<instructions::Redeem>, redeemable_amount: Amount) -> Result<()> {
        instructions::redeem::handler(ctx, redeemable_amount)
    }

    pub fn lock_xxusd(ctx: Context<instructions::LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
        instructions::lock_xxusd::handler(ctx, amount, lock_period)
    }

    pub fn release_xxusd(ctx: Context<instructions::ReleaseXxusd>) -> Result<()> {
        instructions::release_xxusd::handler(ctx)
    }

    pub fn manage_product_price(ctx: Context<instructions::ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
        instructions::manage_product_price::handler(ctx, product_id, price)
    }

    pub fn manage_hedging_strategy(ctx: Context<instructions::ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
        instructions::manage_hedging_strategy::handler(ctx, amount, is_deposit)
    }

    pub fn freeze_program(ctx: Context<instructions::FreezeProgram>, freeze: bool) -> Result<()> {
        instructions::freeze_program::handler(ctx, freeze)
    }

    pub fn edit_controller(ctx: Context<instructions::EditController>, new_authority: Option<Pubkey>) -> Result<()> {
        instructions::edit_controller::handler(ctx, new_authority)
    }
}

pub fn validate_is_program_frozen(controller: &Account<Controller>) -> Result<()> {
    if controller.is_frozen {
        return Err(error::XxusdError::ProgramFrozen.into());
    }
    Ok(())
}

pub fn handle_error<T>(result: Result<T>) -> Result<T> {
    result.map_err(|e| {
        msg!("Error occurred: {:?}", e);
        error::XxusdError::from(e)
    })
}