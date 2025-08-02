use alloy::{
    primitives::{B256, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};

use crate::chain_id::ChainId;

pub trait CustomAlloy {
    fn to_u256(&self) -> U256;
    fn to_b256(&self) -> B256;
}

impl CustomAlloy for B256 {
    fn to_u256(&self) -> U256 {
        U256::from_be_bytes(self.0)
    }

    fn to_b256(&self) -> B256 {
        *self
    }
}

impl CustomAlloy for U256 {
    fn to_u256(&self) -> U256 {
        *self
    }

    fn to_b256(&self) -> B256 {
        B256::from(*self)
    }
}

impl CustomAlloy for Bytes {
    fn to_u256(&self) -> U256 {
        if self.len() > 32 {
            panic!("Expected at most 32 bytes, got {}", self.len());
        }

        let mut word = [0u8; 32];
        word[32 - self.len()..].copy_from_slice(self);

        U256::from_be_bytes(word)
    }

    fn to_b256(&self) -> B256 {
        self.to_u256().to_b256()
    }
}

pub fn create_provider(chain_id: ChainId, wallet: PrivateKeySigner) -> impl Provider {
    let var = match chain_id {
        ChainId::Ethereum => "ETH_RPC_URL",
        ChainId::Optimism => "OPTIMISM_RPC_URL",
        ChainId::Arbitrum => "ARBITRUM_RPC_URL",
        ChainId::Tron => "TRON_RPC_URL",
    };

    let url = std::env::var(var).unwrap().parse().unwrap();

    ProviderBuilder::new().wallet(wallet).connect_http(url)
}

sol! {
     #[sol(rpc)]
    contract ERC20 {
        function balanceOf(address owner) view returns (uint256);
        function allowance(address owner, address spender) view returns (uint256);
        function transfer(address to, uint256 value) returns (bool);
        function approve(address spender, uint256 value) returns (bool);
    }
}
