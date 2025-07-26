use alloy::primitives::Address;

use crate::{
    constants::UINT_16_MAX,
    fusion::{auction_details::AuctionWhitelistItem, fusion_order::IntegratorFee},
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
        assert!(data.whitelist.len() > 0, "whitelist can not be empty");

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
}
