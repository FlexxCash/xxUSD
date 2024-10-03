use anchor_lang::prelude::*;

// - Global Events ------------------------------------------------------------

/// Event called in [instructions::initialize_controller::handler].
#[event]
pub struct InitializeControllerEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller being created.
    #[index]
    pub controller: Pubkey,
    /// The authority.
    pub authority: Pubkey,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRedeemableGlobalSupplyCapEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The new cap.
    pub redeemable_global_supply_cap: u128,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetOutflowLimitPerEpochAmountEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The flat amount redeemable per epoch
    pub outflow_limit_per_epoch_amount: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetOutflowLimitPerEpochBpsEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The portion of supply redeemable per epoch
    pub outflow_limit_per_epoch_bps: u16,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetSlotsPerEpochEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// How many slot for an epoch
    pub slots_per_epoch: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRouterDepositoriesWeightBps {
    #[index]
    pub controller_version: u8,
    #[index]
    pub controller: Pubkey,
    /// The new weights
    pub identity_depository_weight_bps: u16,
    pub mercurial_vault_depository_weight_bps: u16,
    pub credix_lp_depository_weight_bps: u16,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetRouterDepositories {
    #[index]
    pub controller_version: u8,
    #[index]
    pub controller: Pubkey,
    /// The new addresses
    pub identity_depository: Pubkey,
    pub mercurial_vault_depository: Pubkey,
    pub credix_lp_depository: Pubkey,
}

/// Event called in [instructions::freeze_program::handler].
#[event]
pub struct FreezeProgramEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// is program frozen
    pub is_frozen: bool,
    /// reason for freezing
    pub reason: u8,
}

/// Event called in [instructions::lock_xxusd::handler].
#[event]
pub struct LockXxusdEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The amount of xxUSD locked.
    pub amount: u64,
    /// The lock period in seconds.
    pub lock_period: i64,
}

/// Event called in [instructions::release_xxusd::handler].
#[event]
pub struct ReleaseXxusdEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The amount of xxUSD released.
    pub amount: u64,
}

/// Event called in [instructions::redeem::handler].
#[event]
pub struct RedeemEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The user making the call.
    #[index]
    pub user: Pubkey,
    /// The amount of xxUSD redeemed.
    pub xxusd_amount: u64,
    /// The amount of SOL received.
    pub sol_amount: u64,
}

/// Event called in [instructions::manage_product_price::handler].
#[event]
pub struct SetProductPriceEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The product ID.
    pub product_id: u64,
    /// The new price.
    pub price: u64,
}

/// Event called in [instructions::manage_hedging_strategy::handler].
#[event]
pub struct HedgingStrategyEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The type of hedging strategy operation.
    pub operation_type: u8,
    /// The amount involved in the operation.
    pub amount: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetEmergencyShutdownPriceThresholdEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The new emergency shutdown price threshold.
    pub emergency_shutdown_price_threshold: u64,
}

/// Event called in [instructions::edit_controller::handler].
#[event]
pub struct SetCollateralRatioEvent {
    /// The controller version.
    #[index]
    pub version: u8,
    /// The controller.
    #[index]
    pub controller: Pubkey,
    /// The new collateral ratio.
    pub collateral_ratio: u64,
}