pub mod preset;

use alloy::primitives::{Bytes, U256};
use serde::{Deserialize, Serialize};

use crate::{
    chain_id::ChainId,
    fusion::auction_details::AuctionWhitelistItem,
    multichain_address::MultichainAddress,
    quote::preset::{Preset, PresetType},
    time_locks::TimeLocks,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    #[serde(rename = "srcChain")]
    pub src_chain_id: ChainId,
    #[serde(rename = "dstChain")]
    pub dst_chain_id: ChainId,
    pub src_token_address: MultichainAddress,
    pub dst_token_address: MultichainAddress,
    #[serde(rename = "amount")]
    pub src_amount: U256,
    pub enable_estimate: bool,
    #[serde(rename = "walletAddress")]
    pub maker_address: MultichainAddress,
    pub permit: Option<Bytes>,
    pub fee: Option<u64>,
    pub source: Option<String>,
    pub is_permit2: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResult {
    pub quote_id: Option<String>,
    pub src_token_amount: U256,
    pub dst_token_amount: U256,
    pub presets: QuotePresets,
    pub src_escrow_factory: MultichainAddress,
    pub dst_escrow_factory: MultichainAddress,
    pub whitelist: Vec<MultichainAddress>,
    pub time_locks: TimeLocks,
    pub src_safety_deposit: U256,
    pub dst_safety_deposit: U256,
    pub recommended_preset: PresetType,
    pub prices: PairCurrency,
    pub volume: PairCurrency,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePresets {
    pub fast: Preset,
    pub medium: Preset,
    pub slow: Preset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Preset>,
}

#[cfg_attr(test, derive(Default, PartialEq))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCostConfig {
    pub gas_bump_estimate: u64,
    pub gas_price_estimate: U256, // API sends this as a string
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

impl QuoteRequest {
    pub fn new(
        src_chain_id: impl Into<ChainId>,
        dst_chain_id: impl Into<ChainId>,
        src_token_address: impl Into<MultichainAddress>,
        dst_token_address: impl Into<MultichainAddress>,
        src_amount: impl Into<U256>,
        enable_estimate: bool,
        maker_address: impl Into<MultichainAddress>,
    ) -> Self {
        QuoteRequest {
            src_chain_id: src_chain_id.into(),
            dst_chain_id: dst_chain_id.into(),
            src_token_address: src_token_address.into(),
            dst_token_address: dst_token_address.into(),
            src_amount: src_amount.into(),
            enable_estimate,
            maker_address: maker_address.into(),
            permit: None,
            fee: None,
            source: Some("gm/rust-sdk".to_string()),
            is_permit2: None,
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

    pub fn get_whitelist(
        &self,
        auction_start_time: u64,
        exclusive_resolver: Option<&MultichainAddress>,
    ) -> Vec<AuctionWhitelistItem> {
        if let Some(exclusive_resolver) = exclusive_resolver {
            self.whitelist
                .iter()
                .map(|resolver| {
                    let is_exclusive = exclusive_resolver == resolver;
                    AuctionWhitelistItem {
                        address: *resolver,
                        allow_from: if is_exclusive { 0 } else { auction_start_time },
                    }
                })
                .collect()
        } else {
            self.whitelist
                .iter()
                .map(|resolver| AuctionWhitelistItem {
                    address: *resolver,
                    allow_from: 0,
                })
                .collect()
        }
    }
}
