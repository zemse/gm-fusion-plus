pub mod types;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    Error,
    api::types::{ActiveOrder, ActiveOrdersRequestParams, PaginatedParams, PaginationOutput},
    quote::{QuoteRequest, QuoteResult},
    relayer_request::RelayerRequest,
    utils::serde_response_custom_parser::SerdeResponseParse,
};

pub struct Api {
    pub base_url: String,
    pub api_key: String,
}

impl Api {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Api {
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    pub async fn get_quote(&self, params: &QuoteRequest) -> crate::Result<QuoteResult> {
        let result = self
            .perform_get("quoter/v1.0/quote/receive", params)
            .await?;

        Ok(result)
    }

    pub async fn submit_order(&self, relayer_request: RelayerRequest) -> crate::Result<()> {
        self.perform_post("relayer/v1.0/submit", relayer_request)
            .await
    }

    pub async fn get_active_orders(
        &self,
        request: PaginatedParams<ActiveOrdersRequestParams>,
    ) -> crate::Result<PaginationOutput<ActiveOrder>> {
        self.perform_get("orders/v1.0/order/active", request).await
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

    async fn perform_post<B, R>(&self, route: &str, body: B) -> crate::Result<R>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{route}", self.base_url);
        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;
        if result.status().is_success() {
            if result.content_length() == Some(0) {
                let unit: R = serde_json::from_str("null").unwrap();
                Ok(unit)
            } else {
                let response: R = result.serde_parse_custom().await?;
                Ok(response)
            }
        } else {
            let error_text = result.text().await?;
            Err(Error::InternalError(error_text))
        }
    }
}
