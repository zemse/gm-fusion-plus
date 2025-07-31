use alloy::primitives::{B256, Bytes, U256};
use serde::{Deserialize, Serialize, ser::SerializeMap};

use crate::{chain_id::ChainId, limit::eip712::LimitOrderV4};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersRequestParams {
    pub src_chain_id: Option<ChainId>,
    pub dst_chain_id: Option<ChainId>,
}

impl ActiveOrdersRequestParams {
    pub fn paginated(self) -> PaginatedParams<Self> {
        PaginatedParams {
            page: None,
            limit: None,
            inner: self,
        }
    }

    pub fn with_pagination(
        self,
        page: Option<usize>,
        limit: Option<usize>,
    ) -> PaginatedParams<Self> {
        PaginatedParams {
            page,
            limit,
            inner: self,
        }
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

impl Serialize for PaginatedParams<ActiveOrdersRequestParams> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize `inner` to a map
        let inner_value = serde_json::to_value(&self.inner).map_err(serde::ser::Error::custom)?;

        let inner_map = match inner_value {
            serde_json::Value::Object(map) => map,
            _ => {
                return Err(serde::ser::Error::custom(
                    "Expected inner to serialize to a JSON object",
                ));
            }
        };

        // Calculate total fields
        let field_count =
            inner_map.len() + self.page.is_some() as usize + self.limit.is_some() as usize;

        let mut map = serializer.serialize_map(Some(field_count))?;

        // Insert page and limit if present
        if let Some(page) = self.page {
            map.serialize_entry("page", &page)?;
        }
        if let Some(limit) = self.limit {
            map.serialize_entry("limit", &limit)?;
        }

        // Spread out the fields from `inner`
        for (k, v) in inner_map {
            map.serialize_entry(&k, &v)?;
        }

        map.end()
    }
}

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
