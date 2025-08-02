use alloy::primitives::U256;

use crate::{
    fusion::{
        auction_details::{AuctionDetails, AuctionPoint},
        settlement_post_interaction::SettlementPostInteractionData,
    },
    quote::GasCostConfig,
};

pub const RATE_BUMP_DENOMINATOR: u64 = 10_000_000; // 100%
pub const GAS_PRICE_BASE: u64 = 1_000_000;

#[derive(Clone, Debug)]
pub struct AuctionCalculator {
    pub start_time: u64,
    pub duration: u64,
    pub initial_rate_bump: u64,
    pub points: Vec<AuctionPoint>,
    pub taker_fee_ratio: u64,
    pub gas_cost: GasCostConfig,
}

impl AuctionCalculator {
    pub fn from_auction_data(data: SettlementPostInteractionData, details: AuctionDetails) -> Self {
        AuctionCalculator {
            start_time: data.resolving_start_time,
            duration: details.duration,
            initial_rate_bump: details.initial_rate_bump,
            points: details.points,
            taker_fee_ratio: data.integrator_fee.map(|f| f.ratio).unwrap_or(0),
            gas_cost: details.gas_cost,
        }
    }

    pub fn finish_time(&self) -> u64 {
        self.start_time + self.duration
    }

    pub fn calc_initial_rate_bump(start_amount: u64, end_amount: u64) -> u64 {
        (RATE_BUMP_DENOMINATOR * start_amount / end_amount).saturating_sub(RATE_BUMP_DENOMINATOR)
    }

    pub fn calc_auction_taking_amount(taking_amount: U256, rate: u64) -> U256 {
        let numerator = taking_amount * U256::from(rate + RATE_BUMP_DENOMINATOR);
        let denominator = U256::from(RATE_BUMP_DENOMINATOR);
        numerator.div_ceil(denominator) // rounding up
    }

    pub fn base_fee_to_gas_price_estimate(base_fee: u64) -> u64 {
        base_fee / 1_000_000
    }

    pub fn calc_gas_bump_estimate(end_taking_amount: U256, gas_cost_in_to_token: U256) -> U256 {
        (gas_cost_in_to_token * U256::from(RATE_BUMP_DENOMINATOR)) / end_taking_amount
    }

    pub fn calc_rate_bump(&self, time: u64, block_base_fee: U256) -> u64 {
        let gas_bump = self.get_gas_price_bump(block_base_fee);
        let auction_bump = self.get_auction_bump(time);

        auction_bump.saturating_sub(gas_bump)
    }

    fn get_gas_price_bump(&self, block_base_fee: U256) -> u64 {
        if self.gas_cost.gas_bump_estimate == 0
            || self.gas_cost.gas_price_estimate.is_zero()
            || block_base_fee == U256::ZERO
        {
            return 0;
        }

        (((U256::from(self.gas_cost.gas_bump_estimate) * block_base_fee)
            / self.gas_cost.gas_price_estimate)
            / U256::from(1_000_000))
        .to::<u64>()
    }

    fn get_auction_bump(&self, block_time: u64) -> u64 {
        let auction_finish_time = self.finish_time();

        if block_time <= self.start_time {
            return self.initial_rate_bump;
        } else if block_time >= auction_finish_time {
            return 0;
        }

        let mut current_point_time = self.start_time;
        let mut current_rate_bump = self.initial_rate_bump;

        for AuctionPoint { coefficient, delay } in &self.points {
            let next_point_time = current_point_time + *delay;

            if block_time <= next_point_time {
                let time_diff = next_point_time - current_point_time;
                let elapsed = block_time - current_point_time;
                let remaining = next_point_time - block_time;

                return (elapsed * *coefficient + remaining * current_rate_bump) / time_diff;
            }

            current_point_time = next_point_time;
            current_rate_bump = *coefficient;
        }

        let remaining_time = auction_finish_time - block_time;
        let total = auction_finish_time - current_point_time;

        if total == 0 {
            return 0;
        }

        remaining_time * current_rate_bump / total
    }
}

#[cfg(test)]
mod test {
    use alloy::primitives::U256;

    use crate::{
        auction_calculator::AuctionCalculator,
        fusion::{
            auction_details::{AuctionDetails, AuctionPoint},
            fusion_order::IntegratorFee,
            settlement_post_interaction::SettlementPostInteractionData,
        },
        quote::GasCostConfig,
    };

    #[test]
    fn test_auction_calc() {
        let data = SettlementPostInteractionData {
            whitelist: vec![],
            integrator_fee: Some(IntegratorFee {
                receiver: "0x0000000000000000000000000000000000000000"
                    .parse()
                    .unwrap(),
                ratio: 0,
            }),
            bank_fee: Some(0),
            resolving_start_time: 1754118166,
            custom_receiver: Some(
                "0x5bc44f18b91f55540d11d612c08e4faad619eb55"
                    .parse()
                    .unwrap(),
            ),
        };

        let details = AuctionDetails {
            start_time: 1754118183,
            duration: 180,
            initial_rate_bump: 525220,
            points: vec![
                AuctionPoint {
                    delay: 120,
                    coefficient: 380924,
                },
                AuctionPoint {
                    delay: 60,
                    coefficient: 265238,
                },
            ],
            gas_cost: GasCostConfig {
                gas_bump_estimate: 265238,
                gas_price_estimate: U256::from(10),
            },
        };

        let calculator = AuctionCalculator::from_auction_data(data, details);
        let t1 = calculator.get_auction_bump(calculator.start_time);
        println!(
            "t1 = {t1} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), t1)
        );

        let tmid = calculator.get_auction_bump(calculator.start_time + calculator.duration / 4);
        println!(
            "tmid = {tmid} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tmid)
        );

        let tmid = calculator.get_auction_bump(calculator.start_time + calculator.duration / 2);
        println!(
            "tmid = {tmid} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tmid)
        );

        let tmid = calculator.get_auction_bump(calculator.start_time + 120);
        println!(
            "tmid 120 = {tmid} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tmid)
        );

        let tmid = calculator.get_auction_bump(calculator.start_time + calculator.duration * 3 / 4);
        println!(
            "tmid = {tmid} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tmid)
        );

        let tn = calculator.get_auction_bump(calculator.start_time + 179);
        println!(
            "tn = {tn} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tn)
        );

        let tn = calculator.get_auction_bump(calculator.start_time + calculator.duration);
        println!(
            "tn = {tn} {}",
            AuctionCalculator::calc_auction_taking_amount(U256::from(1_000_000), tn)
        );
    }
}
