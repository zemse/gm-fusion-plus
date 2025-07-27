use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

use crate::constants::UINT_32_MAX;

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
        assert!(src_public_cancellation < src_cancellation);
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
