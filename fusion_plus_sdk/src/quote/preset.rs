use alloy::primitives::{Address, U256};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    fusion::auction_details::{AuctionDetails, AuctionPoint},
    quote::GasCostConfig,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PresetType {
    Fast,
    Medium,
    Slow,
    Custom,
}

// https://github.com/1inch/cross-chain-sdk/blob/25ac3927c706a43e85f2f08cc9d9a3bdf156e1e9/src/api/quoter/preset.ts#L4
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub auction_duration: u64,
    pub start_auction_in: u64,
    pub initial_rate_bump: u64,
    pub auction_start_amount: U256,
    pub start_amount: U256,
    pub auction_end_amount: U256,
    pub exclusive_resolver: Option<Address>,
    pub cost_in_dst_token: U256,
    pub points: Vec<AuctionPoint>,
    pub allow_partial_fills: bool,
    pub allow_multiple_fills: bool,
    pub gas_cost: GasCostConfig,
    pub secrets_count: usize,
}

impl Preset {
    pub fn create_auction_details(&self, additional_wait_period: Option<u64>) -> AuctionDetails {
        AuctionDetails::new(
            self.calc_auction_start_time(additional_wait_period),
            self.auction_duration,
            self.initial_rate_bump,
            self.points.clone(),
            self.gas_cost.clone(),
        )
    }

    fn calc_auction_start_time(&self, additional_wait_period: Option<u64>) -> u64 {
        (Utc::now().timestamp() as u64)
            + additional_wait_period.unwrap_or_default()
            + self.start_auction_in
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPreset {
    pub auction_duration: u64,
    pub auction_start_amount: U256,
    pub auction_end_amount: U256,
    pub points: Option<Vec<CustomPresetPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomPresetPoint {
    pub to_token_amount: U256,
    pub delay: u64,
}
