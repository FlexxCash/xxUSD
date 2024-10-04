use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::XxusdError;
use crate::utils::calculate_depositories_sum_value;
use crate::utils::checked_as_u64;
use crate::BPS_POWER;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_fraction_ceil;

pub struct DepositoryInfoForTargetRedeemableAmount {
    pub weight_bps: u16,
    pub redeemable_amount_under_management_cap: u128,
    pub is_jupsol: bool, // New field to identify jupSOL depositories
}

pub fn calculate_depositories_target_redeemable_amount(
    redeemable_circulating_supply: u128,
    depositories_info: &[DepositoryInfoForTargetRedeemableAmount],
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        XxusdError::InvalidDepositoriesVector
    );

    let redeemable_circulating_supply = checked_as_u64(redeemable_circulating_supply)?;

    // Double check that the weights adds up to 100%
    let depositories_weights_bps = depositories_info
        .iter()
        .map(|depository| u64::from(depository.weight_bps))
        .collect::<Vec<u64>>();
    let total_weight_bps = calculate_depositories_sum_value(&depositories_weights_bps)?;
    require!(
        total_weight_bps == BPS_POWER,
        XxusdError::InvalidDepositoriesWeightBps,
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Read the desired weights for each depository on chain
    // -- And generate a raw_target estimations that we can refine later
    // ---------------------------------------------------------------------

    let depositories_raw_target_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            let raw_target = compute_amount_fraction_ceil(
                redeemable_circulating_supply,
                depository.weight_bps.into(),
                BPS_POWER,
            )?;

            // Apply jupSOL specific logic if needed
            if depository.is_jupsol {
                // Here you can add any jupSOL specific calculations
                // For example, you might want to adjust the raw_target based on jupSOL's APY
                // This is just a placeholder, replace with actual logic as needed
                Ok(raw_target.saturating_mul(105).saturating_div(100)) // Example: 5% increase for jupSOL
            } else {
                Ok(raw_target)
            }
        })
        .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Using the raw_target and the depository cap:
    // -- Compute the overflow (raw target amount above the cap)
    // -- Compute the availability (raw target amount until the cap)
    // ---------------------------------------------------------------------

    // Read the minting caps of each depository
    let depositories_hard_cap_amount = depositories_info
        .iter()
        .map(|depository| checked_as_u64(depository.redeemable_amount_under_management_cap))
        .collect::<Result<Vec<u64>>>()?;

    // Compute the depository_overflow amount of raw target that doesn't fit within the cap of each depository
    let depositories_overflow_amount = common_core::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        depositories_hard_cap_amount.iter(),
    )
    .map(
        |(depository_raw_target_redeemable_amount, depository_hard_cap_amount)| {
            if depository_raw_target_redeemable_amount <= depository_hard_cap_amount {
                return Ok(0);
            }
            Ok(depository_raw_target_redeemable_amount
                .checked_sub(*depository_hard_cap_amount)
                .ok_or(XxusdError::MathOverflow)?)
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // Compute the amount of space available under the cap in each depository
    let depositories_available_amount = common_core::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        depositories_hard_cap_amount.iter(),
    )
    .map(
        |(depository_raw_target_redeemable_amount, depository_hard_cap_amount)| {
            if depository_raw_target_redeemable_amount >= depository_hard_cap_amount {
                return Ok(0);
            }
            Ok(depository_hard_cap_amount
                .checked_sub(*depository_raw_target_redeemable_amount)
                .ok_or(XxusdError::MathOverflow)?)
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the combined overflow of all depositories
    // -- Compute the combined availability of all depositories
    // ---------------------------------------------------------------------

    // Compute total amount that doesn't fit within depositories hard cap
    let total_overflow_amount = calculate_depositories_sum_value(&depositories_overflow_amount)?;
    // Compute total amount that doesn't fit within depositories hard cap
    let total_available_amount = calculate_depositories_sum_value(&depositories_available_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Compute the final target based off of the logic:
    // -- Target = raw_target - overflow_amount + Extras
    // -- Extras = total_overflow_amount * (available_amount / total_available_amount)
    // -- In other words:
    // -- The final target is capped at the depository hard cap
    // -- Any amount overflowing that, is allocated to others depositories
    // -- Depositories with available space will receive a portion of allocated overflows
    // ---------------------------------------------------------------------

    // Compute the final targets for each depository
    let depositories_target_redeemable_amount = common_core::iter::zip(
        depositories_raw_target_redeemable_amount.iter(),
        common_core::iter::zip(
            depositories_overflow_amount.iter(),
            common_core::iter::zip(
                depositories_available_amount.iter(),
                depositories_info.iter(),
            ),
        ),
    )
    .map(
        |(
            depository_raw_target_redeemable_amount,
            (depository_overflow_amount, (depository_available_amount, depository_info)),
        )| {
            // Compute the amount of overflow from other depositories that this depository can take
            let overflow_amount_reallocated_from_other_depositories: u64 =
                if total_available_amount > 0 {
                    // We try to rellocate up to the maximum available total amount.
                    // If the overflow amount is more than the available amount, there is nothing we can do
                    let total_amount_reallocatable =
                    common_core::cmp::min(total_overflow_amount, total_available_amount);
                    compute_amount_fraction_ceil(
                        total_amount_reallocatable,
                        *depository_available_amount,
                        total_available_amount,
                    )?
                } else {
                    0
                };
            let mut final_target = depository_raw_target_redeemable_amount
                .checked_add(overflow_amount_reallocated_from_other_depositories)
                .ok_or(XxusdError::MathOverflow)?
                .checked_sub(*depository_overflow_amount)
                .ok_or(XxusdError::MathOverflow)?;

            // Apply jupSOL specific logic if needed
            if depository_info.is_jupsol {
                // Here you can add any jupSOL specific calculations for the final target
                // This is just a placeholder, replace with actual logic as needed
                final_target = final_target.saturating_mul(102).saturating_div(100); // Example: 2% increase for jupSOL
            }

            Ok(final_target)
        },
    )
    .collect::<Result<Vec<u64>>>()?;

    // Done
    Ok(depositories_target_redeemable_amount)
}