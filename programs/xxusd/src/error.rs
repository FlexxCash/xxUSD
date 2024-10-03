use anchor_lang::prelude::*;

#[error_code]
pub enum XxusdError {
    #[msg("The program is frozen")]
    ProgramFrozen,

    #[msg("Invalid collateral amount")]
    InvalidCollateralAmount,

    #[msg("Insufficient collateral balance")]
    InsufficientCollateralBalance,

    #[msg("Invalid redeemable amount")]
    InvalidRedeemableAmount,

    #[msg("Insufficient redeemable balance")]
    InsufficientRedeemableBalance,

    #[msg("Invalid depository")]
    InvalidDepository,

    #[msg("Invalid redeemable mint")]
    InvalidRedeemableMint,

    #[msg("Invalid collateral mint")]
    InvalidCollateralMint,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Invalid controller")]
    InvalidController,

    #[msg("Overflow")]
    Overflow,

    #[msg("Invalid redeemable mint decimals")]
    InvalidRedeemableMintDecimals,

    #[msg("Invalid lock period")]
    InvalidLockPeriod,

    #[msg("Lock period not ended")]
    LockPeriodNotEnded,

    #[msg("Invalid product price")]
    InvalidProductPrice,

    #[msg("Invalid hedging strategy")]
    InvalidHedgingStrategy,

    #[msg("Product not found")]
    ProductNotFound,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Lock not found")]
    LockNotFound,

    #[msg("Insufficient collateral")]
    InsufficientCollateral,

    #[msg("Invalid product ID")]
    InvalidProductId,

    #[msg("Maximum number of products reached")]
    MaxProductsReached,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Insufficient releasable amount")]
    InsufficientReleasableAmount,

    #[msg("Insufficient Balance")]
    InsufficientBalance,
}