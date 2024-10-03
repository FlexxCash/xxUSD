use anchor_lang::prelude::*;

pub mod controller;
pub mod kamino_depository;
pub mod lock_manager;
pub mod hedging_strategy;

pub use controller::*;
pub use kamino_depository::*;
pub use lock_manager::*;
pub use hedging_strategy::*;

/// 表示金額的自定義類型
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Amount(pub u64);

/// 表示時間戳的自定義類型
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
}

impl Default for Amount {
    fn default() -> Self {
        Amount(0)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp(0)
    }
}