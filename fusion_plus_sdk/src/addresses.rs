use crate::{chain_id::ChainId, multichain_address::MultichainAddress};

// TODO update this
pub fn get_limit_order_contract_address(chain_id: ChainId) -> MultichainAddress {
    match chain_id {
        ChainId::Ethereum => "0x111111125421ca6dc452d289314280a0f8842a65"
            .parse()
            .unwrap(),
        ChainId::Optimism => "0x111111125421ca6dc452d289314280a0f8842a65"
            .parse()
            .unwrap(),
        ChainId::Arbitrum => "0x111111125421ca6dc452d289314280a0f8842a65"
            .parse()
            .unwrap(),
        ChainId::Tron => todo!(),
    }
}

pub fn get_true_erc20_address(chain_id: ChainId) -> MultichainAddress {
    match chain_id {
        ChainId::Ethereum => "0xda0000d4000015a526378bb6fafc650cea5966f8"
            .parse()
            .unwrap(),
        ChainId::Optimism => "0xda0000d4000015a526378bb6fafc650cea5966f8"
            .parse()
            .unwrap(),
        ChainId::Arbitrum => "0xda0000d4000015a526378bb6fafc650cea5966f8"
            .parse()
            .unwrap(),
        ChainId::Tron => todo!(),
    }
}

pub fn usdc(chain_id: ChainId) -> MultichainAddress {
    match chain_id {
        ChainId::Ethereum => "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse()
            .unwrap(),
        ChainId::Optimism => "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85"
            .parse()
            .unwrap(),
        ChainId::Arbitrum => "0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
            .parse()
            .unwrap(),
        ChainId::Tron => todo!(),
    }
}

pub fn usdt(chain_id: ChainId) -> MultichainAddress {
    match chain_id {
        ChainId::Ethereum => "0xdAC17F958D2ee523a2206206994597C13D831ec7"
            .parse()
            .unwrap(),
        ChainId::Optimism => "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58"
            .parse()
            .unwrap(),
        ChainId::Arbitrum => "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9"
            .parse()
            .unwrap(),
        ChainId::Tron => todo!(),
    }
}
