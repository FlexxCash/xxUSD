use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt;
use crate::error::XxusdError;

pub trait NumericType: Copy + Clone + BorshSerialize + BorshDeserialize {
    fn to_u64(&self) -> u64;
    fn from_u64(value: u64) -> Self;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct Amount(pub u64);

impl Amount {
    pub fn new(value: u64) -> Self {
        Amount(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn to_u128(&self) -> u128 {
        self.0 as u128
    }

    pub fn from_u128(value: u128) -> Result<Self> {
        Ok(Amount(safe_u128_to_u64(value)?))
    }
}

impl NumericType for Amount {
    fn to_u64(&self) -> u64 {
        self.0
    }

    fn from_u64(value: u64) -> Self {
        Amount(value)
    }
}

impl fmt::Debug for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Amount({})", self.0)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(value: i64) -> Self {
        Timestamp(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

impl NumericType for Timestamp {
    fn to_u64(&self) -> u64 {
        self.0 as u64
    }

    fn from_u64(value: u64) -> Self {
        Timestamp(value as i64)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timestamp({})", self.0)
    }
}

// 輔助函數
pub fn u64_to_amount(value: u64) -> Amount {
    Amount::new(value)
}

pub fn amount_to_u64(amount: Amount) -> u64 {
    amount.value()
}

pub fn i64_to_timestamp(value: i64) -> Timestamp {
    Timestamp::new(value)
}

pub fn timestamp_to_i64(timestamp: Timestamp) -> i64 {
    timestamp.value()
}

// 安全轉換函數
pub fn safe_u128_to_u64(value: u128) -> Result<u64> {
    value.try_into().map_err(|_| XxusdError::Overflow.into())
}

pub fn safe_u64_to_u128(value: u64) -> u128 {
    value as u128
}