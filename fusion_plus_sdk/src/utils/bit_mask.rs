use alloy::primitives::U256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitMask {
    pub offset: u32,
    pub mask: U256,
}

impl BitMask {
    pub fn new(start_bit: u32, end_bit: Option<u32>) -> Self {
        let end_bit = end_bit.unwrap_or(start_bit + 1);

        assert!(
            start_bit < end_bit,
            "BitMask: startBit must be less than endBit"
        );

        Self {
            offset: start_bit,
            mask: (U256::from(1) << (end_bit - start_bit)) - U256::from(1),
        }
    }

    pub fn to_u256(&self) -> U256 {
        self.mask << self.offset
    }

    pub fn to_hex_string(&self) -> String {
        format!("{:#x}", self.to_u256())
    }

    // https://github.com/1inch/ts-byte-utils-lib/blob/53ddb51d47112db52c1f2954743a31cd771e0f37/src/bn/bn.ts#L92
    pub fn get_from(&self, value: U256) -> U256 {
        value >> self.offset & self.mask
    }

    pub fn set_at(&self, principal_value: U256, new_bits: U256) -> U256 {
        (principal_value & !(self.mask << self.offset)) | ((new_bits & self.mask) << self.offset)
    }
}
