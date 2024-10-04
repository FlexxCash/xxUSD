use anchor_lang::prelude::*;
use crate::error::XxusdError;
use std::convert::TryInto;

pub mod controller;
pub mod lock_manager;
pub mod hedging_strategy;
pub mod kamino_depository;

pub use controller::Controller;
pub use lock_manager::LockManager;
pub use hedging_strategy::HedgingStrategy;
pub use kamino_depository::KaminoDepository;

/// 表示金額的自定義類型
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Amount(pub u64);

/// 表示時間戳的自定義類型
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Timestamp(pub i64);

impl Amount {
    /// 創建一個新的 Amount 實例
    pub fn new(value: u64) -> Self {
        Amount(value)
    }

    /// 獲取 Amount 的值
    pub fn value(&self) -> u64 {
        self.0
    }

    /// 從 u128 創建 Amount
    pub fn from_u128(value: u128) -> anchor_lang::Result<Self> {
        Ok(Amount(value.try_into().map_err(|_| XxusdError::Overflow)?))
    }

    /// 將 Amount 轉換為 u128
    pub fn to_u128(&self) -> u128 {
        self.0 as u128
    }

    /// 將 Amount 轉換為 u64
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    /// 從 u64 創建 Amount
    pub fn from_u64(value: u64) -> Self {
        Amount(value)
    }
}

impl Timestamp {
    /// 創建一個新的 Timestamp 實例
    pub fn new(value: i64) -> Self {
        Timestamp(value)
    }

    /// 獲取 Timestamp 的值
    pub fn value(&self) -> i64 {
        self.0
    }

    /// 將 Timestamp 轉換為 u64
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }

    /// 從 u64 創建 Timestamp
    pub fn from_u64(value: u64) -> Self {
        Timestamp(value as i64)
    }
}

/// 將 u64 轉換為 Amount
pub fn u64_to_amount(value: u64) -> Amount {
    Amount::from_u64(value)
}

/// 將 Amount 轉換為 u64
pub fn amount_to_u64(amount: Amount) -> u64 {
    amount.to_u64()
}

/// 將 i64 轉換為 Timestamp
pub fn i64_to_timestamp(value: i64) -> Timestamp {
    Timestamp::new(value)
}

/// 將 Timestamp 轉換為 i64
pub fn timestamp_to_i64(timestamp: Timestamp) -> i64 {
    timestamp.value()
}

/// 安全地將 u128 轉換為 u64
pub fn safe_u128_to_u64(value: u128) -> anchor_lang::Result<u64> {
    Ok(u64::try_from(value).map_err(|_| XxusdError::Overflow)?)
}

/// 將 u64 轉換為 u128
pub fn safe_u64_to_u128(value: u64) -> u128 {
    value as u128
}