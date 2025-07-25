use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::FusionPlusSdk;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteParams {
    #[serde(rename = "srcChain")]
    pub src_chain_id: u32,
    #[serde(rename = "dstChain")]
    pub dst_chain_id: u32,
    pub src_token_address: String,
    pub dst_token_address: String,
    #[serde(rename = "amount")]
    pub src_amount: String,
    pub enable_estimate: bool,
    #[serde(rename = "walletAddress")]
    pub dst_address: String,
}

impl QuoteParams {
    pub fn new(
        src_chain_id: impl Into<u32>,
        dst_chain_id: impl Into<u32>,
        src_token_address: impl ToString,
        dst_token_address: impl ToString,
        src_amount: impl ToString,
        enable_estimate: bool,
        dst_address: impl ToString,
    ) -> Self {
        QuoteParams {
            src_chain_id: src_chain_id.into(),
            dst_chain_id: dst_chain_id.into(),
            src_token_address: src_token_address.to_string(),
            dst_token_address: dst_token_address.to_string(),
            src_amount: src_amount.to_string(),
            enable_estimate,
            dst_address: dst_address.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResult {
    pub quote_id: Value, // QuoteId,
    pub src_token_amount: String,
    pub dst_token_amount: String,
    pub presets: QuotePresets,
    pub src_escrow_factory: String,
    pub dst_escrow_factory: String,
    pub whitelist: Vec<String>,
    pub time_locks: TimeLocks,
    pub src_safety_deposit: String,
    pub dst_safety_deposit: String,
    pub recommended_preset: RecommendedPreset,
    pub prices: PairCurrency,
    pub volume: PairCurrency,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct QuoteId {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePresets {
    pub fast: Preset,
    pub medium: Preset,
    pub slow: Preset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Preset>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub auction_duration: f64,
    pub start_auction_in: f64,
    pub initial_rate_bump: f64,
    pub auction_start_amount: String,
    pub start_amount: String,
    pub auction_end_amount: String,
    pub exclusive_resolver: Value, // ExclusiveResolver,
    pub cost_in_dst_token: String,
    pub points: Vec<AuctionPoint>,
    pub allow_partial_fills: bool,
    pub allow_multiple_fills: bool,
    pub gas_cost: GasCostConfig,
    pub secrets_count: u32,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ExclusiveResolver {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPoint {
    pub delay: f64,
    pub coefficient: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCostConfig {
    pub gas_bump_estimate: f64,
    pub gas_price_estimate: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeLocks {
    pub src_withdrawal: f64,
    pub src_public_withdrawal: f64,
    pub src_cancellation: f64,
    pub src_public_cancellation: f64,
    pub dst_withdrawal: f64,
    pub dst_public_withdrawal: f64,
    pub dst_cancellation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecommendedPreset {
    Fast,
    Medium,
    Slow,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairCurrency {
    pub usd: TokenPair,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub src_token: String,
    pub dst_token: String,
}

impl FusionPlusSdk {
    pub async fn get_quote(&self, params: QuoteParams) -> crate::Result<QuoteResult> {
        let result = self
            .perform_get("quoter/v1.0/quote/receive", params)
            .await?;

        Ok(result)
    }
}
