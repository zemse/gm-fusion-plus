use alloy::primitives::{Address, Bytes, FixedBytes, U128, U160, U256};

#[derive(Debug, Default)]
pub struct BytesBuilder {
    value: Bytes,
}

impl BytesBuilder {
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn push_bytes(&mut self, bytes: Bytes) {
        self.value = [self.value.clone(), bytes].concat().into();
    }

    pub fn push_address(&mut self, address: Address) {
        self.push_bytes(address.into_array().into());
    }

    pub fn push_fixed_bytes<const N: usize>(&mut self, bytes: FixedBytes<N>) {
        self.push_bytes(bytes.into());
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.push_bytes([byte].into());
    }

    pub fn push_uint8(&mut self, value: u8) {
        self.push_bytes(value.to_be_bytes().into());
    }

    pub fn push_uint16(&mut self, value: u16) {
        self.push_bytes(value.to_be_bytes().into());
    }

    pub fn push_uint32(&mut self, value: u32) {
        self.push_bytes(value.to_be_bytes().into());
    }

    pub fn push_uint64(&mut self, value: u64) {
        self.push_bytes(value.to_be_bytes().into());
    }

    pub fn push_uint128(&mut self, value: U128) {
        self.push_bytes(value.to_be_bytes::<16>().into());
    }

    pub fn push_uint160(&mut self, value: U160) {
        self.push_bytes(value.to_be_bytes::<20>().into());
    }

    pub fn push_uint256(&mut self, value: U256) {
        self.push_bytes(value.to_be_bytes::<32>().into());
    }

    pub fn to_u256(&self) -> U256 {
        assert!(
            self.value.len() <= 32,
            "Bytes length must be 32 for U256 conversion"
        );

        let mut value = [0u8; 32];
        value[32 - self.value.len()..].copy_from_slice(&self.value);
        U256::from_be_bytes(value)
    }

    pub fn into_value(self) -> Bytes {
        self.value
    }

    pub fn value(&self) -> &Bytes {
        &self.value
    }
}
