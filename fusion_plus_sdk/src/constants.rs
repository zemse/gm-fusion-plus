use alloy::primitives::{Address, U256};

pub const UINT_8_MAX: u64 = 0xFF;
pub const UINT_16_MAX: u64 = 0xFFFF;
pub const UINT_24_MAX: u64 = 0xFFFFFF;
pub const UINT_32_MAX: u64 = 0xFFFFFFFF;
pub const UINT_40_MAX: u64 = 0xFFFFFFFFFF;

pub const UINT_128_MAX: U256 = U256::from_limbs([
    u64::MAX, // lower 64 bits
    u64::MAX, // upper 64 bits of the 128-bit value
    0,
    0,
]);

pub const UINT_160_MAX: U256 = U256::from_limbs([
    u64::MAX,        // lower 64 bits
    u64::MAX,        // next 64 bits
    u32::MAX as u64, // next 32 bits
    0,
]);

pub const UINT_256_MAX: U256 = U256::from_limbs([
    u64::MAX, // lower 64 bits
    u64::MAX, // next 64 bits
    u64::MAX, // next 64 bits
    u64::MAX, // upper 64 bits of the 256-bit value
]);
pub const NATIVE_CURRENCY: Address = Address::new([
    0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE,
    0xEE, 0xEE, 0xEE, 0xEE,
]);
