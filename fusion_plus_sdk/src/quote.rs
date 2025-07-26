pub mod preset;

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    FusionPlusSdk,
    quote::preset::{Preset, PresetType},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    #[serde(rename = "srcChain")]
    pub src_chain_id: u32,
    #[serde(rename = "dstChain")]
    pub dst_chain_id: u32,
    pub src_token_address: Address,
    pub dst_token_address: Address,
    #[serde(rename = "amount")]
    pub src_amount: String,
    pub enable_estimate: bool,
    #[serde(rename = "walletAddress")]
    pub dst_address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResult {
    pub quote_id: Value, // QuoteId,
    pub src_token_amount: String,
    pub dst_token_amount: String,
    pub presets: QuotePresets,
    pub src_escrow_factory: Address,
    pub dst_escrow_factory: Address,
    pub whitelist: Vec<String>,
    pub time_locks: TimeLocks,
    pub src_safety_deposit: String,
    pub dst_safety_deposit: String,
    pub recommended_preset: PresetType,
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

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ExclusiveResolver {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCostConfig {
    pub gas_bump_estimate: u64,
    pub gas_price_estimate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeLocks {
    pub src_withdrawal: usize,
    pub src_public_withdrawal: usize,
    pub src_cancellation: usize,
    pub src_public_cancellation: usize,
    pub dst_withdrawal: usize,
    pub dst_public_withdrawal: usize,
    pub dst_cancellation: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairCurrency {
    pub usd: TokenPair,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub src_token: Address,
    pub dst_token: Address,
}

impl FusionPlusSdk {
    pub async fn get_quote(&self, params: QuoteRequest) -> crate::Result<QuoteResult> {
        let result = self
            .perform_get("quoter/v1.0/quote/receive", params)
            .await?;

        Ok(result)
    }
}

impl QuoteRequest {
    pub fn new(
        src_chain_id: impl Into<u32>,
        dst_chain_id: impl Into<u32>,
        src_token_address: impl Into<Address>,
        dst_token_address: impl Into<Address>,
        src_amount: impl ToString,
        enable_estimate: bool,
        dst_address: impl Into<Address>,
    ) -> Self {
        QuoteRequest {
            src_chain_id: src_chain_id.into(),
            dst_chain_id: dst_chain_id.into(),
            src_token_address: src_token_address.into(),
            dst_token_address: dst_token_address.into(),
            src_amount: src_amount.to_string(),
            enable_estimate,
            dst_address: dst_address.into(),
        }
    }
}

impl QuoteResult {
    pub fn recommended_preset(&self) -> &Preset {
        self.get_preset(self.recommended_preset)
            .unwrap_or(&self.presets.fast)
    }

    pub fn get_preset(&self, preset_type: PresetType) -> Option<&Preset> {
        match preset_type {
            PresetType::Fast => Some(&self.presets.fast),
            PresetType::Medium => Some(&self.presets.medium),
            PresetType::Slow => Some(&self.presets.slow),
            PresetType::Custom => self.presets.custom.as_ref(),
        }
    }
}
