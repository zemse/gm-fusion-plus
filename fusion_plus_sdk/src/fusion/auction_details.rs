use alloy::{
    dyn_abi::DynSolValue,
    primitives::{Bytes, U256},
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{UINT_24_MAX, UINT_32_MAX},
    limit::extension::Extension,
    multichain_address::MultichainAddress,
    quote::GasCostConfig,
    utils::bytes_iter::{BytesIter, Side},
};

pub struct AuctionWhitelistItem {
    pub address: MultichainAddress,
    // Timestamp in sec at which address can start resolving
    pub allow_from: u64,
}

#[cfg_attr(test, derive(Default, PartialEq))]
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

    pub fn decode_from(bytes: Bytes) -> Self {
        let mut iter = BytesIter::new(bytes);

        let gas_bump_estimate = iter.next_uint24(Side::Front).to::<u64>();
        let gas_price_estimate = iter.next_uint32(Side::Front).to::<u64>();
        let start_time = iter.next_uint32(Side::Front).to::<u64>();
        let duration = iter.next_uint24(Side::Front).to::<u64>();
        let initial_rate_bump = iter.next_uint24(Side::Front).to::<u64>();

        let mut points = vec![];
        while !iter.is_empty() {
            let coefficient = iter.next_uint24(Side::Front).to::<u64>();
            let delay = iter.next_uint16(Side::Front).to::<u64>();

            points.push(AuctionPoint { coefficient, delay });
        }

        AuctionDetails::new(
            start_time,
            duration,
            initial_rate_bump,
            points,
            GasCostConfig {
                gas_bump_estimate,
                gas_price_estimate: U256::from(gas_price_estimate),
            },
        )
    }

    pub fn from_extension(extension: &Extension) -> Self {
        let mut iter = BytesIter::new(extension.making_amount_data.clone());
        let _ = iter.next_address(Side::Front);
        Self::decode_from(iter.rest())
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionPoint {
    pub delay: u64,
    pub coefficient: u64,
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
        assert!(gas_cost.gas_price_estimate.to::<u64>() <= UINT_32_MAX);

        Self {
            start_time,
            duration,
            initial_rate_bump,
            points,
            gas_cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_details_encode_decode() {
        let details = AuctionDetails::new(
            1753727976,
            3600,
            1000,
            vec![
                AuctionPoint {
                    delay: 0,
                    coefficient: 100,
                },
                AuctionPoint {
                    delay: 600,
                    coefficient: 200,
                },
                AuctionPoint {
                    delay: 1200,
                    coefficient: 300,
                },
            ],
            GasCostConfig {
                gas_bump_estimate: 500000,
                gas_price_estimate: U256::from(20000000),
            },
        );

        let encoded = details.encode();
        let decoded = AuctionDetails::decode_from(encoded.clone());

        assert_eq!(details, decoded);
    }
}
