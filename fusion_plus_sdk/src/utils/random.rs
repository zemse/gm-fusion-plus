use alloy::primitives::{B256, U256};
use rand::RngCore;

pub fn get_random_bytes32() -> B256 {
    let mut buf = [0u8; 32];
    rand::rng().fill_bytes(&mut buf);
    B256::from(buf)
}

pub fn get_random_uint<const SIZE: usize>() -> U256 {
    assert!(SIZE <= 32);
    let mut buf = [0u8; SIZE];
    rand::rng().fill_bytes(&mut buf);

    let mut buf_new = [0u8; 32];
    buf_new[32 - SIZE..].copy_from_slice(&buf);

    U256::from_be_slice(&buf_new)
}
