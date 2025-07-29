use alloy::primitives::{B256, Bytes, U256};

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
