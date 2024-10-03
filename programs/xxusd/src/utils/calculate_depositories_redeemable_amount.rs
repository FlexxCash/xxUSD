use anchor_lang::prelude::Result;
use anchor_lang::require;

use crate::error::XxusdError;
use crate::utils::calculate_depositories_sum_value;
use crate::utils::checked_as_u64;
use crate::ROUTER_DEPOSITORIES_COUNT;

use super::compute_amount_less_fraction_floor;

pub struct DepositoryInfoForRedeemableAmount {
    pub is_liquid: bool,
    pub target_redeemable_amount: u64,
    pub redeemable_amount_under_management: u128,
    pub unlock_time: i64,
}

pub fn calculate_depositories_redeemable_amount(
    requested_redeemable_amount: u64,
    depositories_info: &[DepositoryInfoForRedeemableAmount],
    current_time: i64,
) -> Result<Vec<u64>> {
    require!(
        depositories_info.len() == ROUTER_DEPOSITORIES_COUNT,
        XxusdError::InvalidDepositoriesVector
    );

    // ---------------------------------------------------------------------
    // -- Phase 1
    // -- Calculate the available redeemable amount for each depository
    // -- considering the unlock time
    // ---------------------------------------------------------------------

    let depositories_available_redeemable_amount = depositories_info
        .iter()
        .map(|depository| {
            if !depository.is_liquid || current_time < depository.unlock_time {
                return Ok(0);
            }
            checked_as_u64(depository.redeemable_amount_under_management)
        })
        .collect::<Result<Vec<u64>>>()?;

    let total_available_redeemable_amount =
        calculate_depositories_sum_value(&depositories_available_redeemable_amount)?;

    // ---------------------------------------------------------------------
    // -- Phase 2
    // -- Check that we have enough redeemable across all our available methods
    // -- to be able to fulfill the user's redeemable requested amount
    // ---------------------------------------------------------------------

    require!(
        total_available_redeemable_amount >= requested_redeemable_amount,
        XxusdError::InsufficientRedeemableAmount
    );

    // ---------------------------------------------------------------------
    // -- Phase 3
    // -- Compute the final amounts by distributing the requested amount
    // -- proportionally across the available depositories
    // ---------------------------------------------------------------------

    let mut depositories_redeemable_amount = depositories_available_redeemable_amount
        .iter()
        .map(|&available_amount| {
            compute_amount_less_fraction_floor(
                requested_redeemable_amount,
                total_available_redeemable_amount - available_amount,
                total_available_redeemable_amount,
            )
        })
        .collect::<Result<Vec<u64>>>()?;

    // ---------------------------------------------------------------------
    // -- Phase 4
    // -- Correct for precision loss rounding errors
    // ---------------------------------------------------------------------

    let total_redeemable_amount =
        calculate_depositories_sum_value(&depositories_redeemable_amount)?;

    let mut rounding_errors = requested_redeemable_amount
        .checked_sub(total_redeemable_amount)
        .ok_or(XxusdError::MathOverflow)?;

    for i in 0..depositories_info.len() {
        if rounding_errors == 0 {
            break;
        }
        let depository = &depositories_info[i];
        if !depository.is_liquid || current_time < depository.unlock_time {
            continue;
        }
        let depository_remaining_after_redeem = depositories_available_redeemable_amount[i]
            .checked_sub(depositories_redeemable_amount[i])
            .ok_or(XxusdError::MathOverflow)?;
        let depository_rounding_correction =
            std::cmp::min(depository_remaining_after_redeem, rounding_errors);
        depositories_redeemable_amount[i] = depositories_redeemable_amount[i]
            .checked_add(depository_rounding_correction)
            .ok_or(XxusdError::MathOverflow)?;
        rounding_errors = rounding_errors
            .checked_sub(depository_rounding_correction)
            .ok_or(XxusdError::MathOverflow)?;
    }

    // Done
    Ok(depositories_redeemable_amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_depositories_redeemable_amount() {
        let depositories_info = vec![
            DepositoryInfoForRedeemableAmount {
                is_liquid: true,
                target_redeemable_amount: 1000,
                redeemable_amount_under_management: 1500,
                unlock_time: 100,
            },
            DepositoryInfoForRedeemableAmount {
                is_liquid: true,
                target_redeemable_amount: 2000,
                redeemable_amount_under_management: 2500,
                unlock_time: 200,
            },
            DepositoryInfoForRedeemableAmount {
                is_liquid: true,
                target_redeemable_amount: 3000,
                redeemable_amount_under_management: 3500,
                unlock_time: 300,
            },
        ];

        let current_time = 250;
        let requested_redeemable_amount = 3000;

        let result = calculate_depositories_redeemable_amount(
            requested_redeemable_amount,
            &depositories_info,
            current_time,
        )
        .unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 1500);
        assert_eq!(result[1], 1500);
        assert_eq!(result[2], 0);
    }
}