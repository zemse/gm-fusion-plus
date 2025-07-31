pub mod types;

use std::str::FromStr;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{
    Error,
    api::types::{
        ActiveOrder, ActiveOrdersRequestParams, IsEmpty, OrderFillsByMakerOutput,
        OrdersByMakerParams, PaginatedParams, PaginationOutput,
    },
    chain_id::ChainId,
    multichain_address::MultichainAddress,
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

    pub async fn get_orders_by_maker(
        &self,
        maker: MultichainAddress,
        params: PaginatedParams<OrdersByMakerParams>,
    ) -> crate::Result<PaginationOutput<OrderFillsByMakerOutput>> {
        self.perform_get(
            format!("orders/v1.0/order/maker/{}", maker.without_chain_id()).as_str(),
            params,
        )
        .await
    }

    pub async fn get_escrow_factory_contract_address(
        &self,
        chain_id: ChainId,
    ) -> crate::Result<MultichainAddress> {
        let value: Value = self
            .perform_get(
                "orders/v1.0/order/escrow",
                json!({
                    "chainId": chain_id
                }),
            )
            .await?;

        let address = value
            .as_object()
            .and_then(|obj| obj.get("address"))
            .and_then(|value| value.as_str())
            .ok_or(crate::Error::InternalErrorStr(
                "expected address field in response",
            ))?;

        MultichainAddress::from_str(address)
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

#[cfg(test)]
mod tests {
    use crate::chain_id::ChainId;

    #[tokio::test]
    pub async fn test_get_escrow_factory_contract_address() {
        let api = api_sdk();
        let address = api
            .get_escrow_factory_contract_address(ChainId::Arbitrum)
            .await
            .unwrap();
        assert_eq!(
            address.to_string(),
            "0xa7bCb4EAc8964306F9e3764f67Db6A7af6DdF99A"
        );
    }

    fn api_sdk() -> super::Api {
        dotenvy::from_path("../.env").unwrap();

        super::Api::new(
            "https://api.1inch.dev/fusion-plus",
            std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY not set in .env file"),
        )
    }
}
