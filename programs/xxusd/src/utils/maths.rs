use crate::state::{Amount, Timestamp};
use crate::error::XxusdError;
use anchor_lang::prelude::*;

pub fn checked_add(a: Amount, b: Amount) -> Result<Amount> {
    let result = a.value().checked_add(b.value()).ok_or(XxusdError::Overflow)?;
    Ok(Amount::new(result))
}

pub fn checked_sub(a: Amount, b: Amount) -> Result<Amount> {
    let result = a.value().checked_sub(b.value()).ok_or(XxusdError::Overflow)?;
    Ok(Amount::new(result))
}

pub fn checked_sub_timestamp(a: Timestamp, b: Timestamp) -> Result<i64> {
    a.value().checked_sub(b.value()).ok_or(XxusdError::Overflow.into())
}

// 添加其他必要的數學函數