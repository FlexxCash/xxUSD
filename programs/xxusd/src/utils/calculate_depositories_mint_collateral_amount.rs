use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::XxusdError;
use crate::utils::calculate_depositories_sum_value;
use crate::utils::checked_as_u64;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForMintCollateralAmount {
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u128,
    pub is_jupsol: bool, // New field to identify jupSOL depositories
}

pub fn calculate_depositories_mint_collateral_amount(
    requested_mint_collateral_amount: u64,
    depositories_info: &[DepositoryInfoForMintCollateralAmount],
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        XxusdError::InvalidDepositoriesVector
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the maximum mintable collateral amount for each depository
    // ---------------------------------------------------------------------

    let depositories_maximum_mintable_collateral_amount = depositories_info
        .iter()
        .map(|depository| {
            let depository_redeemable_amount_under_management =
                checked_as_u64(depository.redeemable_amount_under_management)?;
            if depository.target_redeemable_amount <= depository_redeemable_amount_under_management
            {
                return Ok(0);
            }
            let max_mintable = depository
                .target_redeemable_amount
                .checked_sub(depository_redeemable_amount_under_management)
                .ok_or(XxusdError::MathOverflow)?;

            // Apply jupSOL specific logic if needed
            if depository.is_jupsol {
                // Here you can add any jupSOL specific calculations
                // For example, you might want to adjust the max_mintable based on jupSOL's APY
                // This is just a placeholder, replace with actual logic as needed
                Ok(max_mintable.saturating_mul(105).saturating_div(100)) // Example: 5% increase for jupSOL
            } else {
                Ok(max_mintable)
            }
        })
        .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Calculate the total amount we could possibly mint
    // -- If this total is not enough, we abort
    // ---------------------------------------------------------------------

    let total_maximum_mintable_collateral_amount =
        calculate_depositories_sum_value(&depositories_maximum_mintable_collateral_amount)?;
    require!(
        total_maximum_mintable_collateral_amount >= requested_mint_collateral_amount,
        XxusdError::DepositoriesTargerRedeemableAmountReached
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Calculate the actual minted amount per depository for the requested mint amount,
    // -- it is a weighted slice of the total mintable amount, scaled by the requested mint amount
    // ---------------------------------------------------------------------

    let depositories_mint_collateral_amount = depositories_maximum_mintable_collateral_amount
        .iter()
        .zip(depositories_info.iter())
        .map(|(depository_mintable_collateral_amount, depository_info)| {
            let other_depositories_maximum_mintable_collateral_amount =
                total_maximum_mintable_collateral_amount
                    .checked_sub(*depository_mintable_collateral_amount)
                    .ok_or(XxusdError::MathOverflow)?;
            let mint_amount = compute_amount_less_fraction_floor(
                requested_mint_collateral_amount,
                other_depositories_maximum_mintable_collateral_amount,
                total_maximum_mintable_collateral_amount,
            )?;

            // Apply jupSOL specific logic if needed
            if depository_info.is_jupsol {
                // Here you can add any jupSOL specific calculations for the final mint amount
                // This is just a placeholder, replace with actual logic as needed
                Ok(mint_amount.saturating_mul(102).saturating_div(100)) // Example: 2% increase for jupSOL
            } else {
                Ok(mint_amount)
            }
        })
        .collect::<Result<Vec<u64>>>()?;

    // Done
    Ok(depositories_mint_collateral_amount)
}