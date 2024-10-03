use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::XxusdError;
use crate::state::{Amount, Timestamp};

pub trait NumericType: Copy + Clone + BorshSerialize + BorshDeserialize {
    fn to_u64(&self) -> u64;
    fn from_u64(value: u64) -> Self;
}

impl NumericType for Amount {
    fn to_u64(&self) -> u64 {
        self.value()
    }

    fn from_u64(value: u64) -> Self {
        Amount::new(value)
    }
}

impl NumericType for Timestamp {
    fn to_u64(&self) -> u64 {
        self.value() as u64
    }

    fn from_u64(value: u64) -> Self {
        Timestamp::new(value as i64)
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