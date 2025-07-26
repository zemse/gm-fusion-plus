use alloy::primitives::{Address, FixedBytes, U256};

use crate::utils::bit_mask::BitMask;

fn allowed_sender_mask() -> BitMask {
    BitMask::new(0, Some(80))
}

fn expiration_mask() -> BitMask {
    BitMask::new(80, Some(120))
}

fn nonce_or_epoch_mask() -> BitMask {
    BitMask::new(120, Some(160))
}

fn series_mask() -> BitMask {
    BitMask::new(160, Some(200))
}

// let ALLOWED_SENDER_MASK: BitMask = BitMask::new(0, Some(80));
// const EXPIRATION_MASK: BitMask = BitMask::new(80, 120);
// const NONCE_OR_EPOCH_MASK: BitMask = BitMask::new(120, 160);
// const SERIES_MASK: BitMask = BitMask::new(160, 200);

const NO_PARTIAL_FILLS_FLAG: usize = 255;
const ALLOW_MULTIPLE_FILLS_FLAG: usize = 254;
const PRE_INTERACTION_CALL_FLAG: usize = 252;
const POST_INTERACTION_CALL_FLAG: usize = 251;
const NEED_CHECK_EPOCH_MANAGER_FLAG: usize = 250;
const HAS_EXTENSION_FLAG: usize = 249;
const USE_PERMIT2_FLAG: usize = 248;
const UNWRAP_WETH_FLAG: usize = 247;

#[derive(Default)]
pub struct MakerTraits {
    value: U256,
}

impl MakerTraits {
    pub fn new(value: U256) -> Self {
        Self { value }
    }

    // https://github.com/1inch/ts-byte-utils-lib/blob/53ddb51d47112db52c1f2954743a31cd771e0f37/src/bn/bn.ts#L92
    pub fn get_mask(&self, mask: BitMask) -> U256 {
        self.value >> mask.offset & mask.mask
    }

    // TODO add test for this
    fn set_mask(&self, mask: BitMask, val: U256) -> U256 {
        (self.value & !(mask.mask << mask.offset)) | ((val & mask.mask) << mask.offset)
    }

    pub fn allowed_sender(&self) -> FixedBytes<10> {
        let c: [u8; 10] = self.get_mask(allowed_sender_mask()).to_be_bytes::<32>()[22..]
            .try_into()
            .unwrap();
        FixedBytes::from(c)
    }

    pub fn is_private(&self) -> bool {
        !self.get_mask(allowed_sender_mask()).is_zero()
    }

    pub fn with_allowed_sender(mut self, sender: Address) -> Self {
        assert!(
            sender != Address::ZERO,
            "Use with_any_sender to remove sender check"
        );
        let last_10 = &sender.as_slice()[10..];
        let val = U256::from_be_slice(
            &[0u8; 22]
                .iter()
                .chain(last_10)
                .copied()
                .collect::<Vec<u8>>(),
        );
        self.value = self.set_mask(allowed_sender_mask(), val);
        self
    }

    pub fn with_any_sender(mut self) -> Self {
        self.value = self.set_mask(allowed_sender_mask(), U256::ZERO);
        self
    }

    pub fn expiration(&self) -> u64 {
        let val = self.get_mask(expiration_mask());
        val.to::<u64>()
    }

    pub fn with_expiration(mut self, expiration: u64) -> Self {
        self.value = self.set_mask(expiration_mask(), U256::from(expiration));
        self
    }

    pub fn nonce_or_epoch(&self) -> u64 {
        self.get_mask(nonce_or_epoch_mask()).to::<u64>()
    }

    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.value = self.set_mask(nonce_or_epoch_mask(), U256::from(nonce));
        self
    }

    pub fn series(&self) -> u64 {
        self.get_mask(series_mask()).to::<u64>()
    }

    pub fn with_series(mut self, series: u64) -> Self {
        self.value = self.set_mask(series_mask(), U256::from(series));
        self
    }

    pub fn has_extension(&self) -> bool {
        self.value.bit(HAS_EXTENSION_FLAG)
    }

    pub fn with_extension(mut self) -> Self {
        self.value.set_bit(HAS_EXTENSION_FLAG, true);
        self
    }

    pub fn is_partial_fill_allowed(&self) -> bool {
        !self.value.bit(NO_PARTIAL_FILLS_FLAG)
    }

    pub fn allow_partial_fills(mut self) -> Self {
        self.value.set_bit(NO_PARTIAL_FILLS_FLAG, false);
        self
    }

    pub fn disable_partial_fills(mut self) -> Self {
        self.value.set_bit(NO_PARTIAL_FILLS_FLAG, true);
        self
    }

    pub fn set_partial_fills(self, allow: bool) -> Self {
        if allow {
            self.allow_partial_fills()
        } else {
            self.disable_partial_fills()
        }
    }

    pub fn is_multiple_fills_allowed(&self) -> bool {
        self.value.bit(ALLOW_MULTIPLE_FILLS_FLAG)
    }

    pub fn allow_multiple_fills(mut self) -> Self {
        self.value.set_bit(ALLOW_MULTIPLE_FILLS_FLAG, true);
        self
    }

    pub fn disable_multiple_fills(mut self) -> Self {
        self.value.set_bit(ALLOW_MULTIPLE_FILLS_FLAG, false);
        self
    }

    pub fn set_multiple_fills(self, allow: bool) -> Self {
        if allow {
            self.allow_multiple_fills()
        } else {
            self.disable_multiple_fills()
        }
    }

    pub fn has_pre_interaction(&self) -> bool {
        self.value.bit(PRE_INTERACTION_CALL_FLAG)
    }

    pub fn enable_pre_interaction(mut self) -> Self {
        self.value.set_bit(PRE_INTERACTION_CALL_FLAG, true);
        self
    }

    pub fn disable_pre_interaction(mut self) -> Self {
        self.value.set_bit(PRE_INTERACTION_CALL_FLAG, false);
        self
    }

    pub fn has_post_interaction(&self) -> bool {
        self.value.bit(POST_INTERACTION_CALL_FLAG)
    }

    pub fn enable_post_interaction(mut self) -> Self {
        self.value.set_bit(POST_INTERACTION_CALL_FLAG, true);
        self
    }

    pub fn disable_post_interaction(mut self) -> Self {
        self.value.set_bit(POST_INTERACTION_CALL_FLAG, false);
        self
    }

    pub fn is_epoch_manager_enabled(&self) -> bool {
        self.value.bit(NEED_CHECK_EPOCH_MANAGER_FLAG)
    }

    pub fn is_permit2(&self) -> bool {
        self.value.bit(USE_PERMIT2_FLAG)
    }

    pub fn enable_permit2(mut self) -> Self {
        self.value.set_bit(USE_PERMIT2_FLAG, true);
        self
    }

    pub fn disable_permit2(mut self) -> Self {
        self.value.set_bit(USE_PERMIT2_FLAG, false);
        self
    }

    pub fn is_native_unwrap_enabled(&self) -> bool {
        self.value.bit(UNWRAP_WETH_FLAG)
    }

    pub fn enable_native_unwrap(mut self) -> Self {
        self.value.set_bit(UNWRAP_WETH_FLAG, true);
        self
    }

    pub fn disable_native_unwrap(mut self) -> Self {
        self.value.set_bit(UNWRAP_WETH_FLAG, false);
        self
    }

    pub fn as_u256(&self) -> U256 {
        self.value
    }

    pub fn is_bit_invalidator_mode(&self) -> bool {
        !self.is_partial_fill_allowed() || !self.is_multiple_fills_allowed()
    }

    pub fn with_epoch(self, series: u64, epoch: u64) -> Self {
        let mut this = self.with_series(series);
        this = this.with_nonce(epoch);
        this.enable_epoch_manager_check()
    }

    fn enable_epoch_manager_check(mut self) -> Self {
        assert!(
            self.is_partial_fill_allowed() && self.is_multiple_fills_allowed(),
            "Epoch manager allowed only when partialFills and multipleFills enabled"
        );
        self.value.set_bit(NEED_CHECK_EPOCH_MANAGER_FLAG, true);
        self
    }
}
