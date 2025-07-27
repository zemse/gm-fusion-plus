use alloy::primitives::B256;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    Error,
    cross_chain_order::{CrossChainOrder, CrossChainOrderParams},
    quote::{QuoteRequest, QuoteResult},
    utils::serde_response_custom_parser::SerdeResponseParse,
};

pub struct FusionPlusSdk {
    pub base_url: String,
    pub api_key: String,
}

impl FusionPlusSdk {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        FusionPlusSdk {
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    async fn perform_get<Q, R>(&self, route: &str, params: Q) -> crate::Result<R>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{route}", self.base_url);
        let client = reqwest::Client::new();
        let result = client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(&params)
            .send()
            .await?;
        if result.status().is_success() {
            let response: R = result.serde_parse_custom().await?;
            Ok(response)
        } else {
            let error_text = result.text().await?;
            Err(Error::InternalError(error_text))
        }
    }

    pub async fn get_quote(&self, params: &QuoteRequest) -> crate::Result<QuoteResult> {
        let result = self
            .perform_get("quoter/v1.0/quote/receive", params)
            .await?;

        Ok(result)
    }

    pub async fn place_order(&self, quote: QuoteResult, order_params: CrossChainOrderParams) {}
}
