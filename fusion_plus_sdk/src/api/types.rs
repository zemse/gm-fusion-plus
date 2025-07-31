use alloy::primitives::{B256, Bytes, U256};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};
use serde_json::Value;

use crate::{
    chain_id::ChainId, fusion::auction_details::AuctionPoint, limit::eip712::LimitOrderV4,
    multichain_address::MultichainAddress,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    total_items: usize,
    items_per_page: usize,
    total_pages: usize,
    current_page: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOutput<Inner> {
    meta: PaginationMeta,
    items: Vec<Inner>,
}

#[macro_export]
macro_rules! impl_paginated {
    ($t:ty) => {
        impl $t {
            pub fn paginated(self) -> $crate::api::types::PaginatedParams<Self> {
                $crate::api::types::PaginatedParams {
                    page: None,
                    limit: None,
                    inner: self,
                }
            }

            pub fn with_pagination(
                self,
                page: Option<usize>,
                limit: Option<usize>,
            ) -> $crate::api::types::PaginatedParams<Self> {
                $crate::api::types::PaginatedParams {
                    page,
                    limit,
                    inner: self,
                }
            }
        }
    };
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersByMakerParams {
    pub src_chain: Option<ChainId>,
    pub dst_chain: Option<ChainId>,
    pub src_token: Option<MultichainAddress>,
    pub dst_token: Option<MultichainAddress>,
    pub with_token: Option<MultichainAddress>,
    pub timestamp_from: Option<u64>,
    pub timestamp_to: Option<u64>,
}
impl_paginated!(OrdersByMakerParams);
impl IsEmpty for OrdersByMakerParams {
    fn is_empty(&self) -> bool {
        self.src_chain.is_none()
            && self.dst_chain.is_none()
            && self.src_token.is_none()
            && self.dst_token.is_none()
            && self.with_token.is_none()
            && self.timestamp_from.is_none()
            && self.timestamp_to.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValidationStatus {
    Valid,
    OrderPredicateReturnedFalse,
    NotEnoughBalance,
    NotEnoughAllowance,
    InvalidPermitSignature,
    InvalidPermitSpender,
    InvalidPermitSigner,
    InvalidSignature,
    FailedToParsePermitDetails,
    UnknownPermitVersion,
    WrongEpochManagerAndBitInvalidator,
    FailedToDecodeRemainingMakerAmount,
    UnknownFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Executed,
    Expired,
    Cancelled,
    Refunding,
    Refunded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub status: FillStatus,
    pub tx_hash: String,
    pub filled_maker_amount: String,
    pub filled_auction_taker_amount: String,
    pub escrow_events: Vec<EscrowEventData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FillStatus {
    Pending,
    Executed,
    Refunding,
    Refunded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscrowEventSide {
    Src,
    Dst,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscrowEventAction {
    SrcEscrowCreated,
    DstEscrowCreated,
    Withdrawn,
    FundsRescued,
    EscrowCancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscrowEventData {
    pub transaction_hash: String,
    pub escrow: String,
    pub side: EscrowEventSide,
    pub action: EscrowEventAction,
    pub block_timestamp: u64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFillsByMakerOutput {
    order_hash: String,
    validation: ValidationStatus,
    status: OrderStatus,
    maker_asset: String,
    taker_asset: String,
    maker_amount: String,
    min_taker_amount: String,
    approximate_taking_amount: String,
    cancel_tx: Option<String>,
    fills: Vec<Fill>,
    points: Option<Vec<AuctionPoint>>,
    auction_start_date: u64,
    auction_duration: u64,
    initial_rate_bump: u64,
    #[serde(default)]
    is_native_currency: bool,
    src_chain_id: ChainId,
    dst_chain_id: ChainId,
    created_at: u64,
    cancelable: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersRequestParams {
    pub src_chain_id: Option<ChainId>,
    pub dst_chain_id: Option<ChainId>,
}
impl_paginated!(ActiveOrdersRequestParams);
impl IsEmpty for ActiveOrdersRequestParams {
    fn is_empty(&self) -> bool {
        self.src_chain_id.is_none() && self.dst_chain_id.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrder {
    quote_id: String,
    order_hash: B256,
    signature: Bytes,
    deadline: String,
    auction_start_date: String,
    auction_end_date: String,
    remaining_maker_amount: U256,
    maker_balance: U256,
    maker_allowance: U256,
    order: LimitOrderV4,
    extension: Bytes,
    src_chain_id: ChainId,
    dst_chain_id: ChainId,
    is_maker_contract: bool,
    secret_hashes: Option<Vec<B256>>,
    fills: Vec<FillInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    tx_hash: B256,
}

#[derive(Debug)]
pub struct PaginatedParams<Inner> {
    page: Option<usize>,
    limit: Option<usize>,
    inner: Inner,
}

impl<Inner> Serialize for PaginatedParams<Inner>
where
    Inner: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert inner to serde_json::Value and ensure it's a map
        let inner_value = serde_json::to_value(&self.inner).map_err(serde::ser::Error::custom)?;

        let inner_map = match inner_value {
            Value::Object(map) => map,
            _ => {
                return Err(serde::ser::Error::custom(
                    "Expected inner to serialize to a JSON object",
                ));
            }
        };

        let field_count =
            inner_map.len() + self.page.is_some() as usize + self.limit.is_some() as usize;

        let mut map = serializer.serialize_map(Some(field_count))?;

        if let Some(page) = self.page {
            map.serialize_entry("page", &page)?;
        }
        if let Some(limit) = self.limit {
            map.serialize_entry("limit", &limit)?;
        }

        for (k, v) in inner_map {
            map.serialize_entry(&k, &v)?;
        }

        map.end()
    }
}
