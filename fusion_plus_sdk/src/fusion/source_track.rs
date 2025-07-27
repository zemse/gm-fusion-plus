use alloy::{
    hex,
    primitives::{U256, keccak256},
};

use crate::utils::bit_mask::BitMask;

fn track_code_mask() -> BitMask {
    BitMask::new(224, Some(256))
}

fn get_track_code_for_source(source: &str) -> U256 {
    if !hex::check_raw(source) {
        return create_id(source);
    }

    if source.len() == 10 {
        return U256::from_str_radix(source.trim_start_matches("0x"), 16)
            .expect("Invalid hex string");
    }

    if source.len() == 66 {
        let (source, _) = source.split_at(10);
        return U256::from_str_radix(source.trim_start_matches("0x"), 16)
            .expect("Invalid hex string");
    }

    create_id(source)
}

fn create_id(source: &str) -> U256 {
    let hash = keccak256(source.as_bytes());
    let first_5 = &hash[0..5];

    let mut full_bytes = [0u8; 32];
    full_bytes[32 - 5..].copy_from_slice(first_5);
    U256::from_be_bytes(full_bytes)
}

pub fn inject_track_code(salt: U256, source: Option<String>) -> U256 {
    let track = if let Some(source) = source {
        get_track_code_for_source(&source)
    } else {
        U256::ZERO
    };

    track_code_mask().set_at(salt, track)
}
