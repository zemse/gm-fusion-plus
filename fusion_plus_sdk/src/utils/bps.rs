use alloy::primitives::U256;

// https://github.com/1inch/limit-order-sdk/blob/1793d32bd36c6cfea909caafbc15e8023a033249/src/bps.ts#L8
#[derive(Clone, Debug)]
pub struct Bps {
    value: u64, // Basis points
}

const FEE_BASE: u64 = 100_000;
const BPS_BASE: u64 = 10_000;
const BPS_TO_RATIO_NUMERATOR: u64 = FEE_BASE / BPS_BASE;

impl Bps {
    pub fn to_ratio_format(bps: Option<u64>) -> u64 {
        let Some(bps) = bps else {
            return 0;
        };

        bps * BPS_TO_RATIO_NUMERATOR
    }
}

pub fn add_ratio_to_amount(amount: U256, ratio: u64) -> U256 {
    return amount + ((amount * U256::from(ratio)) / U256::from(FEE_BASE));
}
