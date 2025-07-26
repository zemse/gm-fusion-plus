use alloy::primitives::FixedBytes;

use crate::constants::UINT_16_MAX;

// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/whitelist/types.ts#L1
#[derive(Clone, Debug)]
pub struct WhitelistItem {
    // last 10 bytes of the address
    pub address_half: FixedBytes<10>,
    pub delay: u64,
}

// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/whitelist/whitelist.ts#L9
pub struct Whitelist {
    resolving_start_time: u64,
    whitelist: Vec<WhitelistItem>,
}

impl Whitelist {
    pub fn new(resolving_start_time: u64, whitelist: Vec<WhitelistItem>) -> Self {
        assert!(whitelist.len() > 0, "whitelist cannot be empty");

        whitelist.iter().for_each(|item| {
            assert!(item.delay < UINT_16_MAX, "too big diff between timestamps");
        });

        Self {
            resolving_start_time,
            whitelist,
        }
    }

    pub fn len(&self) -> usize {
        self.whitelist.len()
    }
}
