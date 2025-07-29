use alloy::primitives::{Address, Bytes, U256};

use crate::utils::alloy::CustomAlloy;

pub enum Side {
    Front,
    Back,
}

pub struct BytesIter {
    bytes: Bytes,
}

impl BytesIter {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn rest(self) -> Bytes {
        self.bytes
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn next_byte(&mut self, side: Side) -> Bytes {
        self.next_bytes(1, side)
    }

    pub fn next_bytes(&mut self, n: usize, side: Side) -> Bytes {
        let cnt = n;
        if self.bytes.len() < cnt {
            panic!("Can not consume {n} bytes, have only {}", self.bytes.len());
        }

        let (chunk, rest) = match side {
            Side::Front => {
                let (chunk, rest) = self.bytes.split_at(cnt);
                (chunk, rest)
            }
            Side::Back => {
                let (rest, chunk) = self.bytes.split_at(self.bytes.len() - cnt);
                (chunk, rest)
            }
        };

        let chunk = chunk.to_vec().into();
        self.bytes = rest.to_vec().into();
        chunk
    }

    pub fn next_address(&mut self, side: Side) -> Address {
        let val = self.next_bytes(20, side);
        Address::from_slice(&val)
    }

    pub fn next_uint8(&mut self, side: Side) -> U256 {
        self.next_bytes(1, side).to_u256()
    }

    pub fn next_uint16(&mut self, side: Side) -> U256 {
        self.next_bytes(2, side).to_u256()
    }

    pub fn next_uint24(&mut self, side: Side) -> U256 {
        self.next_bytes(3, side).to_u256()
    }

    pub fn next_uint32(&mut self, side: Side) -> U256 {
        self.next_bytes(4, side).to_u256()
    }

    pub fn next_uint128(&mut self, side: Side) -> U256 {
        self.next_bytes(16, side).to_u256()
    }

    pub fn next_uint160(&mut self, side: Side) -> U256 {
        self.next_bytes(20, side).to_u256()
    }

    pub fn next_uint256(&mut self, side: Side) -> U256 {
        self.next_bytes(32, side).to_u256()
    }

    pub fn first_address(value: Bytes) -> Address {
        let mut value = Self::new(value);
        value.next_address(Side::Front)
    }
}
