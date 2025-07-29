use alloy::primitives::{Address, address};

use crate::chain_id::ChainId;

// TODO update this
pub fn get_limit_order_contract_address(chain_id: ChainId) -> Address {
    match chain_id {
        ChainId::Ethereum => address!("0x111111125421ca6dc452d289314280a0f8842a65"),
        ChainId::Arbitrum => address!("0x111111125421ca6dc452d289314280a0f8842a65"),
    }
}

pub fn get_true_erc20_address(chain_id: ChainId) -> Address {
    match chain_id {
        ChainId::Ethereum => address!("0xda0000d4000015a526378bb6fafc650cea5966f8"),
        ChainId::Arbitrum => address!("0xda0000d4000015a526378bb6fafc650cea5966f8"),
    }
}
