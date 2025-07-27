pub mod error;
pub mod escrow_extension;
pub mod time_locks;
pub use error::{Error, Result};
pub mod addresses;
pub mod chain_id;
pub mod constants;
pub mod fusion;
pub mod hash_lock;
pub mod limit;
pub mod order;
pub mod quote;
pub mod utils;
pub mod whitelist;

use serde::{Serialize, de::DeserializeOwned};

use crate::utils::serde_response_custom_parser::SerdeResponseParse;

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
}
