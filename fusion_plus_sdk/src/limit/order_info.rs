use alloy::primitives::{Address, U256};

pub struct OrderInfoData {
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker: Address,
    pub receiver: Option<Address>,
    pub salt: Option<U256>,
}

impl OrderInfoData {
    pub fn with_receiver(mut self, receiver: Address) -> Self {
        self.receiver = Some(receiver);
        self
    }

    pub fn with_salt(mut self, salt: U256) -> Self {
        self.salt = Some(salt);
        self
    }

    pub fn with_taker_asset(mut self, taker_asset: Address) -> Self {
        self.taker_asset = taker_asset;
        self
    }
}
