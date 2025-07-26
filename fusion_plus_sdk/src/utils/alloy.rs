use alloy::primitives::{B256, U256};

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
