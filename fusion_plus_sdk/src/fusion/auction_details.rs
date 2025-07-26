use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{UINT_24_MAX, UINT_32_MAX},
    quote::GasCostConfig,
};

pub struct AuctionWhitelistItem {
    pub address: Address,
    // Timestamp in sec at which address can start resolving
    pub allow_from: u64,
}

#[derive(Clone, Debug)]
pub struct AuctionDetails {
    pub start_time: u64,
    pub duration: u64,
    pub initial_rate_bump: u64,
    pub points: Vec<AuctionPoint>,
    pub gas_cost: GasCostConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPoint {
    pub delay: u64,
    pub coefficient: usize,
}

impl AuctionDetails {
    pub fn new(
        start_time: u64,
        duration: u64,
        initial_rate_bump: u64,
        points: Vec<AuctionPoint>,
        gas_cost: GasCostConfig,
    ) -> Self {
        assert!(start_time <= UINT_32_MAX);
        assert!(duration <= UINT_24_MAX);
        assert!(initial_rate_bump <= UINT_24_MAX);
        assert!(gas_cost.gas_bump_estimate <= UINT_24_MAX);
        assert!(gas_cost.gas_price_estimate <= UINT_32_MAX);

        Self {
            start_time,
            duration,
            initial_rate_bump,
            points,
            gas_cost,
        }
    }
}
