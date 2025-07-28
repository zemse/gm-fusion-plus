use alloy::{
    primitives::{B256, Bytes},
    signers::Signature,
};
use serde::Serialize;

use crate::{
    chain_id::ChainId,
    cross_chain_order::PreparedOrder,
    limit::{eip712::LimitOrderV4, extension::ExtensionBuildable},
};

#[derive(Debug, Clone, Serialize)]
pub struct RelayerRequest {
    pub src_chain_id: ChainId,
    pub order: LimitOrderV4,
    pub signature: Signature,
    pub quote_id: String,
    pub extension: Bytes,
    pub secret_hashes: Option<Vec<B256>>,
}

impl RelayerRequest {
    pub fn from_prepared_order(
        prepared_order: &PreparedOrder,
        signature: Signature,
        quote_id: String,
        secret_hashes: Option<Vec<B256>>,
    ) -> Self {
        RelayerRequest {
            src_chain_id: prepared_order.src_chain_id,
            order: prepared_order.to_v4(),
            signature,
            quote_id,
            extension: prepared_order.order.inner.extension.build().encode(),
            secret_hashes,
        }
    }
}
