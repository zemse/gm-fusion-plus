use alloy::primitives::{Address, B256, Bytes, U256};

use crate::{
    chain_id::ChainId,
    constants::{NATIVE_CURRENCY, UINT_128_MAX},
    fusion::{
        auction_details::AuctionDetails,
        fusion_extension::{self, FusionExtension},
        settlement_post_interaction::SettlementPostInteractionData,
    },
    hash_lock::HashLock,
    limit::{limit_order::LimitOrder, maker_traits::MakerTraits, order_info::OrderInfoData},
    quote::TimeLocks,
    utils::bps::Bps,
};

// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/surplus-params.ts#L5
pub struct SurplusParams {
    estimated_taker_amount: U256,
    protocol_fee: Bps,
}

#[derive(Clone, Debug)]
pub struct ResolverFee {
    receiver: Address,
    fee: Bps,
}

#[derive(Clone, Debug)]
pub struct IntegratorFee {
    receiver: Address,
    ratio: u64,
}

#[derive(Clone, Debug)]
pub struct Fees {
    resolver: ResolverFee,
    integrator: IntegratorFee,
}

pub struct Interaction {
    pub target: Address,
    pub data: Bytes,
}

#[derive(Clone, Debug)]
pub struct FusionOrderExtra {
    unwrap_weth: Option<bool>,
    nonce: Option<u64>,
    permit: Option<Bytes>,
    allow_partial_fills: Option<bool>,
    allow_multiple_fills: Option<bool>,
    order_expiration_delay: Option<u64>,
    enable_permit2: Option<bool>,
    source: Option<String>,
    fees: Option<Fees>,
}

pub struct FusionOrder {
    settlement_extension_contract: Address,
    // order_info: OrderInfoData,
    // auction_details: AuctionDetails,
    // post_interaction_data: SettlementPostInteractionData,
    // extra: FusionOrderExtra,
    fusion_extension: FusionExtension,
    inner: LimitOrder,
}

impl FusionOrder {
    pub fn new(
        settlement_extension_contract: Address,
        order_info: OrderInfoData,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        extra: Option<FusionOrderExtra>,
        fusion_extension: Option<FusionExtension>,
    ) -> Self {
        let extra_default = FusionOrderExtra {
            unwrap_weth: Some(false),
            nonce: None,
            permit: None,
            allow_partial_fills: Some(true),
            allow_multiple_fills: Some(true),
            order_expiration_delay: Some(12),
            enable_permit2: Some(false),
            source: None,
            fees: None,
        };

        let extra = extra.unwrap_or(extra_default.clone());
        let fusion_extension = fusion_extension.unwrap_or_else(|| FusionExtension {
            settlement_extension_contract,
            auction_details: auction_details.clone(),
            post_interaction_data: post_interaction_data.clone(),
            maker_permit: extra.permit.map(|permit| Interaction {
                target: order_info.maker_asset,
                data: permit,
            }),
        });

        let unwrap_weth = extra.unwrap_weth.or(extra_default.unwrap_weth).unwrap();
        let allow_partial_fills = extra
            .allow_partial_fills
            .or(extra_default.allow_multiple_fills)
            .unwrap();
        let allow_multiple_fills = extra
            .allow_multiple_fills
            .or(extra_default.allow_multiple_fills)
            .unwrap();
        let order_expiration_delay = extra
            .order_expiration_delay
            .or(extra_default.order_expiration_delay)
            .unwrap();
        let enable_permit2 = extra
            .enable_permit2
            .or(extra_default.enable_permit2)
            .unwrap();

        let deadline =
            auction_details.start_time * auction_details.duration + order_expiration_delay;

        let mut maker_traits = MakerTraits::default()
            .with_expiration(deadline)
            .set_partial_fills(allow_partial_fills)
            .set_multiple_fills(allow_multiple_fills)
            .enable_post_interaction();

        if maker_traits.is_bit_invalidator_mode() {
            assert!(
                extra.nonce.is_some(),
                "Nonce required, when partial fill or multiple fill disallowed"
            )
        }

        if unwrap_weth {
            maker_traits = maker_traits.enable_native_unwrap()
        }

        if enable_permit2 {
            maker_traits = maker_traits.enable_permit2()
        }

        if let Some(nonce) = extra.nonce {
            maker_traits = maker_traits.with_nonce(nonce);
        }

        let receiver = if post_interaction_data
            .integrator_fee
            .map(|integrator_fee| integrator_fee.ratio)
            .unwrap_or_default()
            > 0
        {
            settlement_extension_contract
        } else {
            order_info.receiver.unwrap_or(order_info.maker)
        };

        // TODO Build extension

        Self {
            settlement_extension_contract,
            fusion_extension,
            inner: todo!(),
        }
    }
}
