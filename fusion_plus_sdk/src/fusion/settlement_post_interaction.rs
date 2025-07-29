use alloy::primitives::{Address, Bytes, U256};

use crate::{
    fusion::{auction_details::AuctionWhitelistItem, fusion_order::IntegratorFee},
    limit::extension::Extension,
    utils::{
        bit_mask::BitMask,
        bytes_builder::BytesBuilder,
        bytes_iter::{BytesIter, Side},
    },
    whitelist::{WhitelistItem, WhitelistItemIntermediate},
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
#[cfg_attr(test, derive(Default, PartialEq))]
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
        let mut whitelist: Vec<WhitelistItemIntermediate> = data
            .whitelist
            .iter()
            .map(|d| WhitelistItemIntermediate {
                address_half: d.address.as_slice()[10..].try_into().unwrap(),
                // note: delay currently stores a timestamp, actual "delay" secs value is calculated later
                allow_from: if d.allow_from < data.resolving_start_time {
                    data.resolving_start_time
                } else {
                    d.allow_from
                },
            })
            .collect();

        whitelist.sort_by_key(|a| a.allow_from);

        let mut sum_delay = 0;
        let whitelist = whitelist
            .into_iter()
            .map(|item| {
                let delay = item.allow_from - data.resolving_start_time - sum_delay;
                sum_delay += delay;
                WhitelistItem {
                    address_half: item.address_half,
                    delay,
                }
            })
            .collect();

        SettlementPostInteractionData {
            whitelist,
            integrator_fee: data.integrator_fee,
            bank_fee: data.bank_fee,
            resolving_start_time: data.resolving_start_time,
            custom_receiver: data.custom_receiver,
        }
    }

    pub fn encode(&self) -> Bytes {
        let mut flags = U256::ZERO; // u8
        let mut bytes = BytesBuilder::default();

        if let Some(bank_fee) = self.bank_fee {
            if bank_fee > 0 {
                flags.set_bit(0, true);
                bytes.push_uint32(bank_fee as u32);
            }
        }

        if let Some(integrator_fee) = &self.integrator_fee {
            if integrator_fee.ratio > 0 {
                flags.set_bit(1, true);
                bytes.push_uint16(integrator_fee.ratio);
                bytes.push_address(integrator_fee.receiver);

                if let Some(custom_receiver) = self.custom_receiver {
                    flags.set_bit(2, true);
                    bytes.push_address(custom_receiver);
                }
            }
        }

        bytes.push_uint32(self.resolving_start_time as u32);

        for wl in &self.whitelist {
            bytes.push_fixed_bytes(wl.address_half);
            bytes.push_uint16(wl.delay as u16);
        }

        flags = BitMask::new(3, Some(8)).set_at(flags, U256::from(self.whitelist.len()));

        bytes.push_uint8(flags.try_into().expect("bit mask over u8")); // TODO remove expect

        bytes.into_value()
    }

    pub fn decode_from(bytes: Bytes) -> Self {
        let mut iter = BytesIter::new(bytes);
        let flags = iter.next_uint8(Side::Back);

        let mut bank_fee = None;
        let mut integrator_fee = None;
        let mut custom_receiver = None;

        if flags.bit(0) {
            bank_fee = Some(iter.next_uint32(Side::Front).to::<u64>());
        }

        if flags.bit(1) {
            let ratio = iter.next_uint16(Side::Front).to::<u16>();
            let receiver = iter.next_address(Side::Front);
            integrator_fee = Some(IntegratorFee { ratio, receiver });

            if flags.bit(2) {
                custom_receiver = Some(iter.next_address(Side::Front));
            }
        }

        let resolving_start_time = iter.next_uint32(Side::Front).to::<u64>();

        let mut whitelist = vec![];
        while !iter.is_empty() {
            let address_half_bytes = iter.next_bytes(10, Side::Front);
            let mut addess_half = [0u8; 10];
            addess_half.copy_from_slice(&address_half_bytes);

            let delay = iter.next_uint16(Side::Front).to::<u64>();

            whitelist.push(WhitelistItem {
                address_half: addess_half.into(),
                delay,
            });
        }

        Self {
            whitelist,
            integrator_fee,
            bank_fee,
            resolving_start_time,
            custom_receiver,
        }
    }

    pub fn from_extension(extension: &Extension) -> Self {
        let mut iter = BytesIter::new(extension.post_interaction.clone());
        let _ = iter.next_address(Side::Front);

        Self::decode_from(iter.rest())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let post_interaction = SettlementPostInteractionData {
            whitelist: vec![
                WhitelistItem {
                    address_half: "0xcb4fa6eb00f6ea887a4a".parse().unwrap(),
                    delay: 0,
                },
                WhitelistItem {
                    address_half: "0x72f8a0c8c415454f629c".parse().unwrap(),
                    delay: 0,
                },
                WhitelistItem {
                    address_half: "0x5ba74b09ae44e823cf77".parse().unwrap(),
                    delay: 0,
                },
            ],
            integrator_fee: Some(IntegratorFee {
                receiver: "0x0000000000000000000000000000000000000000"
                    .parse()
                    .unwrap(),
                ratio: 1, // Note that if ratio is 0 then integrator_fee will be None
            }),
            bank_fee: Some(1), // Note that if bank_fee is 0 then it will be None
            resolving_start_time: 1753727976,
            custom_receiver: Some(
                "0x0000000000000000000000000000000000000000"
                    .parse()
                    .unwrap(),
            ),
        };

        let encoded = post_interaction.encode();
        let decoded = SettlementPostInteractionData::decode_from(encoded.clone());

        assert_eq!(post_interaction, decoded);
    }
}
