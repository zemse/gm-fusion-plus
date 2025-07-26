use alloy::primitives::{Address, B256, U256};

pub struct LimitOrder {
    salt: B256,
    maker: Address,
    receiver: Address,
    maker_asset: Address,
    taker_asset: Address,
    making_amount: U256,
    taking_amount: U256,
    maker_traits: B256,
}
