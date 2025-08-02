use alloy::primitives::U256;

use crate::multichain_address::MultichainAddress;

pub struct OrderInfoData {
    pub maker_asset: MultichainAddress,
    pub taker_asset: MultichainAddress,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker: MultichainAddress,
    pub receiver: Option<MultichainAddress>,
    pub salt: Option<U256>,
}

impl OrderInfoData {
    pub fn with_receiver(mut self, receiver: MultichainAddress) -> Self {
        self.receiver = Some(receiver);
        self
    }

    pub fn with_salt(mut self, salt: U256) -> Self {
        self.salt = Some(salt);
        self
    }

    pub fn with_taker_asset(mut self, taker_asset: MultichainAddress) -> Self {
        self.taker_asset = taker_asset;
        self
    }
}
