use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use crate::error::XxusdError;

pub fn validate_collateral_amount(
    user_collateral: &Account<TokenAccount>,
    collateral_amount: u64,
) -> Result<()> {
    if collateral_amount == 0 {
        return Err(XxusdError::InvalidCollateralAmount.into());
    }

    if user_collateral.amount < collateral_amount {
        return Err(XxusdError::InsufficientCollateralBalance.into());
    }

    Ok(())
}