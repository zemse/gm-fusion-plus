use alloy::{
    primitives::{B256, Bytes},
    signers::Signature,
};
use serde::Serialize;

use crate::{
    chain_id::ChainId,
    cross_chain_order::PreparedOrder,
    limit::{eip712::LimitOrderV4, extension_builder::ExtensionBuildable},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayerRequest {
    pub src_chain_id: ChainId,
    pub order: LimitOrderV4,
    pub signature: Bytes,
    pub quote_id: String,
    pub extension: Bytes,
    pub secret_hashes: Option<Vec<B256>>,
}

impl RelayerRequest {
    pub fn from_prepared_order(
        prepared_order: &PreparedOrder,
        signature: &Signature,
        quote_id: String,
        secret_hashes: Option<Vec<B256>>,
    ) -> Self {
        let order = prepared_order.to_v4();

        assert_eq!(
            signature
                .recover_address_from_prehash(&order.get_order_hash(prepared_order.src_chain_id))
                .unwrap(),
            order.maker,
            "Signature does not match order maker"
        );

        RelayerRequest {
            src_chain_id: prepared_order.src_chain_id,
            order,
            signature: signature.as_bytes().into(),
            quote_id,
            extension: prepared_order.order.inner.extension.build().encode(),
            secret_hashes,
        }
    }

    pub fn order_hash(&self) -> B256 {
        self.order.get_order_hash(self.src_chain_id)
    }
}
