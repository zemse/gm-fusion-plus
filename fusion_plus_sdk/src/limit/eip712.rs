use std::fmt::Debug;

use alloy::{
    dyn_abi::Eip712Domain,
    primitives::{B256, keccak256},
    sol,
    sol_types::{SolStruct, eip712_domain},
};

use crate::{addresses::get_limit_order_contract_address, chain_id::ChainId};

pub fn get_limit_order_v4_domain(chain_id: ChainId) -> Eip712Domain {
    let verifying_contract = get_limit_order_contract_address(chain_id);

    eip712_domain! {
        name: "1inch Aggregation Router",
        version: "6",
        chain_id: chain_id as u64,
        verifying_contract: verifying_contract,
        salt: keccak256("test"),
    }
}

sol! {
    #[derive(Debug, serde::Serialize)]
    struct LimitOrderV4 {
        uint256 salt;
        address maker;
        address receiver;
        address makerAsset;
        address takerAsset;
        uint256 makingAmount;
        uint256 takingAmount;
        uint256 makerTraits;
    }
}

impl LimitOrderV4 {
    pub fn get_order_hash(&self, chain_id: ChainId) -> B256 {
        let domain = get_limit_order_v4_domain(chain_id);
        self.eip712_signing_hash(&domain)
    }
}

pub trait OrderBuildable: Clone + Debug {
    fn build(&self) -> LimitOrderV4;
}
