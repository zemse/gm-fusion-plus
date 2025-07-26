use alloy::{
    dyn_abi::DynSolValue,
    primitives::{Address, Bytes, U256},
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{UINT_24_MAX, UINT_32_MAX},
    quote::GasCostConfig,
};

pub struct AuctionWhitelistItem {
    pub address: Address,
    // Timestamp in sec at which address can start resolving
    pub allow_from: u64,
}

#[derive(Clone, Debug)]
pub struct AuctionDetails {
    pub start_time: u64,
    pub duration: u64,
    pub initial_rate_bump: u64,
    pub points: Vec<AuctionPoint>,
    pub gas_cost: GasCostConfig,
}

impl AuctionDetails {
    pub fn encode(&self) -> Bytes {
        let mut result = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from(self.gas_cost.gas_bump_estimate), 24),
            DynSolValue::Uint(U256::from(self.gas_cost.gas_price_estimate), 32),
            DynSolValue::Uint(U256::from(self.start_time), 32),
            DynSolValue::Uint(U256::from(self.duration), 24),
            DynSolValue::Uint(U256::from(self.initial_rate_bump), 24),
        ])
        .abi_encode_packed();

        for point in &self.points {
            let point_encoded = DynSolValue::Tuple(vec![
                DynSolValue::Uint(U256::from(point.coefficient), 24),
                DynSolValue::Uint(U256::from(point.delay), 16),
            ])
            .abi_encode_packed();

            result.extend(point_encoded);
        }

        result.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionPoint {
    pub delay: u64,
    pub coefficient: usize,
}

impl AuctionDetails {
    pub fn new(
        start_time: u64,
        duration: u64,
        initial_rate_bump: u64,
        points: Vec<AuctionPoint>,
        gas_cost: GasCostConfig,
    ) -> Self {
        assert!(start_time <= UINT_32_MAX);
        assert!(duration <= UINT_24_MAX);
        assert!(initial_rate_bump <= UINT_24_MAX);
        assert!(gas_cost.gas_bump_estimate <= UINT_24_MAX);
        assert!(gas_cost.gas_price_estimate <= U256::from(UINT_32_MAX));

        Self {
            start_time,
            duration,
            initial_rate_bump,
            points,
            gas_cost,
        }
    }
}
