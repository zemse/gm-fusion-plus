use alloy::primitives::{Address, B256, Bytes, U256};

use crate::{
    chain_id::ChainId,
    fusion::{
        auction_details::AuctionDetails, fusion_extension::FusionExtension,
        settlement_post_interaction::SettlementPostInteractionData,
        source_track::inject_track_code,
    },
    limit::{
        extension_builder::ExtensionBuildable, interaction::Interaction, limit_order::LimitOrder,
        maker_traits::MakerTraits, order_info::OrderInfoData,
    },
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

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug)]
pub struct IntegratorFee {
    pub receiver: Address,
    pub ratio: u64,
}

#[derive(Clone, Debug)]
pub struct Fees {
    resolver: ResolverFee,
    integrator: IntegratorFee,
}

#[derive(Clone, Debug)]
pub struct FusionOrderExtra {
    pub unwrap_weth: Option<bool>,
    pub nonce: Option<u64>,
    pub permit: Option<Bytes>,
    pub allow_partial_fills: Option<bool>,
    pub allow_multiple_fills: Option<bool>,
    pub order_expiration_delay: Option<u64>,
    pub enable_permit2: Option<bool>,
    pub source: Option<String>,
    pub fees: Option<Fees>,
}

#[derive(Clone, Debug)]
pub struct FusionOrder<E: ExtensionBuildable> {
    pub settlement_extension_contract: Address,
    pub extension: E,
    pub inner: LimitOrder,
}

impl FusionOrder<FusionExtension> {
    pub fn new(
        settlement_extension_contract: Address,
        order_info: OrderInfoData,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        extra: Option<FusionOrderExtra>,
    ) -> Self {
        let maker_permit = extra
            .as_ref()
            .and_then(|extra| extra.permit.as_ref())
            .map(|permit| Interaction {
                target: order_info.maker_asset,
                data: permit.clone(),
            });
        Self::new_with_extension(
            settlement_extension_contract,
            order_info,
            auction_details.clone(),
            post_interaction_data.clone(),
            extra,
            FusionExtension::new(
                settlement_extension_contract,
                auction_details.clone(),
                post_interaction_data,
                maker_permit,
            ),
        )
    }
}

impl<E: ExtensionBuildable> FusionOrder<E> {
    pub fn new_with_extension(
        settlement_extension_contract: Address,
        order_info: OrderInfoData,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        extra: Option<FusionOrderExtra>,
        extension: E,
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

        let built_extension = extension.build();

        let salt = LimitOrder::build_salt(&built_extension, order_info.salt);
        let salt_with_injected_track_code = if let Some(salt) = order_info.salt {
            salt
        } else {
            inject_track_code(salt, extra.source)
        };

        let inner = LimitOrder::new(
            order_info
                .with_receiver(receiver)
                .with_salt(salt_with_injected_track_code),
            Some(maker_traits),
            Some(built_extension),
        );

        Self {
            settlement_extension_contract,
            extension,
            inner,
        }
    }

    pub fn get_order_hash(&self, chain_id: ChainId) -> B256 {
        self.inner.get_order_hash(chain_id)
    }
}
