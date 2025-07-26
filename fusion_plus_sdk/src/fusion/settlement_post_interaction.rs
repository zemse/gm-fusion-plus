use alloy::primitives::{Address, Bytes, U256};

use crate::{
    constants::UINT_16_MAX,
    fusion::{auction_details::AuctionWhitelistItem, fusion_order::IntegratorFee},
    utils::{bit_mask::BitMask, bytes_builder::BytesBuilder},
    whitelist::WhitelistItem,
};

// https://github.com/1inch/fusion-sdk/blob/32733a8b1d77ad6018591aa93eb162c3995ded20/src/fusion-order/settlement-post-interaction-data/types.ts#L11-L12
pub struct SettlementSuffixData {
    pub whitelist: Vec<AuctionWhitelistItem>,
    pub integrator_fee: Option<IntegratorFee>,
    pub bank_fee: Option<u64>,
    pub resolving_start_time: u64,
    pub custom_receiver: Option<Address>,
}

// https://github.com/1inch/fusion-sdk/blob/32733a8b1d77ad6018591aa93eb162c3995ded20/src/fusion-order/settlement-post-interaction-data/settlement-post-interaction-data.ts#L10-L11
#[derive(Clone, Debug)]
pub struct SettlementPostInteractionData {
    pub whitelist: Vec<WhitelistItem>,
    pub integrator_fee: Option<IntegratorFee>,
    pub bank_fee: Option<u64>,
    pub resolving_start_time: u64,
    pub custom_receiver: Option<Address>,
}

impl SettlementPostInteractionData {
    pub fn new(data: SettlementSuffixData) -> Self {
        assert!(!data.whitelist.is_empty(), "whitelist can not be empty");

        // transfrom timestamps to cumulative delays
        let mut whitelist: Vec<WhitelistItem> = data
            .whitelist
            .iter()
            .map(|d| WhitelistItem {
                address_half: d.address.as_slice()[10..].try_into().unwrap(),
                delay: if d.allow_from < data.resolving_start_time {
                    data.resolving_start_time
                } else {
                    d.allow_from
                },
            })
            .collect();

        whitelist.sort_by_key(|a| a.delay);

        let mut sum_delay = 0;
        whitelist.iter_mut().for_each(|item| {
            item.delay = item.delay - data.resolving_start_time - sum_delay;
            sum_delay += item.delay;
            assert!(item.delay < UINT_16_MAX, "too big diff between timestamps");
        });

        SettlementPostInteractionData {
            whitelist,
            integrator_fee: data.integrator_fee,
            bank_fee: data.bank_fee,
            resolving_start_time: data.resolving_start_time,
            custom_receiver: data.custom_receiver,
        }
    }

    pub fn encode(&self) -> Bytes {
        let mut bit_mask = U256::ZERO; // u8
        let mut bytes = BytesBuilder::default();

        if let Some(bank_fee) = self.bank_fee {
            bit_mask.set_bit(0, true);
            bytes.push_uint32(bank_fee as u32);
        }

        if let Some(integrator_fee) = &self.integrator_fee {
            bit_mask.set_bit(1, true);
            bytes.push_uint16(integrator_fee.ratio);
            bytes.push_address(integrator_fee.receiver);

            if let Some(custom_receiver) = self.custom_receiver {
                bit_mask.set_bit(2, true);
                bytes.push_address(custom_receiver);
            }
        }

        bytes.push_uint32(self.resolving_start_time as u32);

        for wl in &self.whitelist {
            bytes.push_fixed_bytes(wl.address_half);
            bytes.push_uint16(wl.delay as u16);
        }

        bit_mask = BitMask::new(3, Some(8)).set_at(bit_mask, U256::from(self.whitelist.len()));

        bytes.push_uint8(bit_mask.try_into().expect("bit mask over u8")); // TODO remove expect

        bytes.into_value()
    }
}
