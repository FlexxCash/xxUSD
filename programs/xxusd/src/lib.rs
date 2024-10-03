use anchor_lang::prelude::*;
use crate::instructions::*;
#[macro_use]
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

pub struct XxusdProgram;

impl XxusdProgram {
    pub fn initialize_controller(ctx: Context<InitializeController>, redeemable_mint_decimals: u8) -> Result<()> {
        instructions::initialize_controller(ctx, redeemable_mint_decimals)
    }

    pub fn mint(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
        instructions::mint(ctx, collateral_amount)
    }

    pub fn redeem(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
        instructions::redeem(ctx, redeemable_amount)
    }

    pub fn lock_xxusd(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
        instructions::lock_xxusd(ctx, amount, lock_period)
    }

    pub fn release_xxusd(ctx: Context<ReleaseXxusd>) -> Result<()> {
        instructions::release_xxusd(ctx)
    }

    pub fn manage_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
        instructions::manage_product_price(ctx, product_id, price)
    }

    pub fn manage_hedging_strategy(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
        instructions::manage_hedging_strategy(ctx, amount, is_deposit)
    }

    pub fn freeze_program(ctx: Context<FreezeProgram>, freeze: bool) -> Result<()> {
        instructions::freeze_program(ctx, freeze)
    }

    pub fn edit_controller(ctx: Context<EditController>, new_authority: Option<Pubkey>) -> Result<()> {
        instructions::edit_controller(ctx, new_authority)
    }
}

#[program]
pub mod xxusd {
    use super::*;

    pub fn initialize_controller(ctx: Context<InitializeController>, redeemable_mint_decimals: u8) -> Result<()> {
        XxusdProgram::initialize_controller(ctx, redeemable_mint_decimals)
    }

    pub fn mint(ctx: Context<MintInstruction>, collateral_amount: Amount) -> Result<()> {
        XxusdProgram::mint(ctx, collateral_amount)
    }

    pub fn redeem(ctx: Context<Redeem>, redeemable_amount: Amount) -> Result<()> {
        XxusdProgram::redeem(ctx, redeemable_amount)
    }

    pub fn lock_xxusd(ctx: Context<LockXxusd>, amount: Amount, lock_period: Timestamp) -> Result<()> {
        XxusdProgram::lock_xxusd(ctx, amount, lock_period)
    }

    pub fn release_xxusd(ctx: Context<ReleaseXxusd>) -> Result<()> {
        XxusdProgram::release_xxusd(ctx)
    }

    pub fn manage_product_price(ctx: Context<ManageProductPrice>, product_id: u64, price: Amount) -> Result<()> {
        XxusdProgram::manage_product_price(ctx, product_id, price)
    }

    pub fn manage_hedging_strategy(ctx: Context<ManageHedgingStrategy>, amount: Amount, is_deposit: bool) -> Result<()> {
        XxusdProgram::manage_hedging_strategy(ctx, amount, is_deposit)
    }

    pub fn freeze_program(ctx: Context<FreezeProgram>, freeze: bool) -> Result<()> {
        XxusdProgram::freeze_program(ctx, freeze)
    }

    pub fn edit_controller(ctx: Context<EditController>, new_authority: Option<Pubkey>) -> Result<()> {
        XxusdProgram::edit_controller(ctx, new_authority)
    }
}

pub fn validate_is_program_frozen(controller: &Account<Controller>) -> Result<()> {
    if controller.is_frozen {
        return Err(error::XxusdError::ProgramFrozen.into());
    }
    Ok(())
}

pub fn handle_error<T>(result: std::result::Result<T, error::XxusdError>) -> Result<T> {
    result.map_err(|e| {
        msg!("Error occurred: {:?}", e);
        Error::from(e)
    })
}