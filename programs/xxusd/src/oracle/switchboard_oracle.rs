use anchor_lang::prelude::*;
use switchboard_v2::AggregatorAccountData;
use crate::error::XxusdError;

pub struct SwitchboardOracle;

impl SwitchboardOracle {
    pub fn get_jupsol_price(aggregator: &AccountInfo) -> Result<f64> {
        let feed = AggregatorAccountData::new(aggregator)?;
        let price = feed.get_result()?.try_into()?;
        Ok(price)
    }

    pub fn get_jupsol_apy(aggregator: &AccountInfo) -> Result<f64> {
        let feed = AggregatorAccountData::new(aggregator)?;
        let apy = feed.get_result()?.try_into()?;
        Ok(apy)
    }

    pub fn get_sol_price(aggregator: &AccountInfo) -> Result<f64> {
        let feed = AggregatorAccountData::new(aggregator)?;
        let price = feed.get_result()?.try_into()?;
        Ok(price)
    }

    pub fn validate_price_feed(aggregator: &AccountInfo) -> Result<()> {
        let feed = AggregatorAccountData::new(aggregator)?;
        
        // Check if the feed is updated recently (e.g., within the last hour)
        let staleness_threshold = 3600; // 1 hour in seconds
        let current_timestamp = Clock::get()?.unix_timestamp;
        let last_update_timestamp = feed.latest_confirmed_round.round_open_timestamp;
        
        if current_timestamp - last_update_timestamp > staleness_threshold {
            return Err(XxusdError::StalePriceFeed.into());
        }

        Ok(())
    }
}