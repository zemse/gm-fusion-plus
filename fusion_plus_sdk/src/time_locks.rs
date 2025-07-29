use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

use crate::constants::UINT_32_MAX;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeLocks {
    src_withdrawal: u64,
    src_public_withdrawal: u64,
    src_cancellation: u64,
    src_public_cancellation: u64,
    dst_withdrawal: u64,
    dst_public_withdrawal: u64,
    dst_cancellation: u64,
    #[serde(default)]
    deployed_at: u64,
}

impl TimeLocks {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        src_withdrawal: u64,
        src_public_withdrawal: u64,
        src_cancellation: u64,
        src_public_cancellation: u64,
        dst_withdrawal: u64,
        dst_public_withdrawal: u64,
        dst_cancellation: u64,
        deployed_at: Option<u64>,
    ) -> Self {
        let deployed_at = deployed_at.unwrap_or(0);

        assert!(deployed_at <= UINT_32_MAX);
        assert!(src_withdrawal <= UINT_32_MAX);
        assert!(src_withdrawal < src_public_withdrawal);
        assert!(src_public_withdrawal <= UINT_32_MAX);
        assert!(src_public_withdrawal < src_cancellation);
        assert!(src_cancellation <= UINT_32_MAX);
        assert!(src_cancellation < src_public_cancellation);
        assert!(src_public_cancellation <= UINT_32_MAX);
        assert!(dst_withdrawal <= UINT_32_MAX);
        assert!(dst_withdrawal < dst_public_withdrawal);
        assert!(dst_public_withdrawal <= UINT_32_MAX);
        assert!(dst_public_withdrawal < dst_cancellation);
        assert!(dst_cancellation <= UINT_32_MAX);

        Self {
            deployed_at,
            src_withdrawal,
            src_public_withdrawal,
            src_cancellation,
            src_public_cancellation,
            dst_withdrawal,
            dst_public_withdrawal,
            dst_cancellation,
        }
    }

    pub fn from_u256(mut value: U256) -> Self {
        let mut parts = [0u64; 8];

        for part in parts.as_mut() {
            *part = (value & U256::from(UINT_32_MAX)).to::<u64>();
            value >>= 32;
        }

        TimeLocks {
            src_withdrawal: parts[0],
            src_public_withdrawal: parts[1],
            src_cancellation: parts[2],
            src_public_cancellation: parts[3],
            dst_withdrawal: parts[4],
            dst_public_withdrawal: parts[5],
            dst_cancellation: parts[6],
            deployed_at: parts[7],
        }
    }

    pub fn build(&self) -> U256 {
        let mut value = U256::ZERO;

        for prop in [
            self.deployed_at,
            self.dst_cancellation,
            self.dst_public_withdrawal,
            self.dst_withdrawal,
            self.src_public_cancellation,
            self.src_cancellation,
            self.src_public_withdrawal,
            self.src_withdrawal,
        ] {
            value = (value << 32) | U256::from(prop);
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_locks_build() {
        let time_locks = TimeLocks::new(36, 372, 528, 648, 60, 336, 456, Some(80));
        let built = time_locks.build();
        let decoded = TimeLocks::from_u256(built);
        assert_eq!(time_locks, decoded);
    }
}
