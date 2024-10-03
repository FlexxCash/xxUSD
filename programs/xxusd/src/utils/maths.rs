use crate::core::{Amount, Timestamp};
use crate::error::XxusdError;

pub fn checked_add(a: Amount, b: Amount) -> Result<Amount, XxusdError> {
    a.value().checked_add(b.value())
        .map(Amount::new)
        .ok_or(XxusdError::MathOverflow)
}

pub fn checked_sub(a: Amount, b: Amount) -> Result<Amount, XxusdError> {
    a.value().checked_sub(b.value())
        .map(Amount::new)
        .ok_or(XxusdError::MathOverflow)
}

pub fn checked_mul(a: Amount, b: u64) -> Result<Amount, XxusdError> {
    a.value().checked_mul(b)
        .map(Amount::new)
        .ok_or(XxusdError::MathOverflow)
}

pub fn checked_div(a: Amount, b: u64) -> Result<Amount, XxusdError> {
    a.value().checked_div(b)
        .map(Amount::new)
        .ok_or(XxusdError::MathOverflow)
}

pub fn checked_as_u64(value: Amount) -> Result<u64, XxusdError> {
    Ok(value.value())
}

pub fn checked_add_timestamp(a: Timestamp, b: i64) -> Result<Timestamp, XxusdError> {
    a.value().checked_add(b)
        .map(Timestamp::new)
        .ok_or(XxusdError::MathOverflow)
}

pub fn checked_sub_timestamp(a: Timestamp, b: Timestamp) -> Result<i64, XxusdError> {
    a.value().checked_sub(b.value())
        .ok_or(XxusdError::MathOverflow)
}