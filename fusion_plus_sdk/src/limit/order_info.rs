use alloy::primitives::{Address, B256, U256};

pub struct OrderInfoData {
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker: Address,
    pub receiver: Option<Address>,
    pub salt: Option<B256>,
}

pub struct LimitOrderV4Struct {
    salt: B256,
    maker: Address,
    receiver: Address,
    maker_asset: Address,
    taker_asset: Address,
    making_amount: U256,
    taking_amount: U256,
    maker_traits: B256,
}
