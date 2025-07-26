use std::ops::Add;

use alloy::primitives::{Address, B256, Bytes, FixedBytes, U256};
use chrono::Utc;
use rand::Rng;
use serde::Serialize;

use crate::{
    FusionPlusSdk,
    chain_id::ChainId,
    constants::{NATIVE_CURRENCY, UINT_40_MAX, UINT_128_MAX},
    fusion::{
        auction_details::{AuctionDetails, AuctionWhitelistItem},
        fusion_order::{FusionOrder, IntegratorFee, Interaction},
        settlement_post_interaction::{SettlementPostInteractionData, SettlementSuffixData},
    },
    hash_lock::HashLock,
    limit::order_info::OrderInfoData,
    quote::{QuoteRequest, QuoteResult, TimeLocks, preset::PresetType},
    utils::bps::Bps,
    whitelist::Whitelist,
};

#[derive(Debug, Serialize)]
pub struct OrderParams {
    #[serde(rename = "walletAddress")]
    dst_address: Address,

    #[serde(rename = "hashLock")]
    hash_lock: HashLock,

    secret_hashes: Vec<B256>,
}

pub struct Fee {
    pub taking_fee_bps: u16,
    pub taking_fee_receiver: Address,
}

impl FusionPlusSdk {
    pub async fn create_order(
        &self,
        quote: QuoteResult,
        order_params: OrderParams,
    ) -> crate::Result<()> {
        if !quote.quote_id.is_null() {
            return Err(crate::Error::InternalErrorStr(
                "request quote with enableEstimate=true",
            ));
        }

        Ok(())
    }
    pub async fn place_order(&self, quote: QuoteResult, order_params: OrderParams) {}
}

// https://github.com/1inch/cross-chain-sdk/blob/25ac3927c706a43e85f2f08cc9d9a3bdf156e1e9/src/api/quoter/quote/types.ts#L5
pub struct CrossChainOrderParamsData {
    hash_lock: HashLock,
    preset: Option<PresetType>,
    receiver: Option<Address>,
    nonce: Option<u64>,
    permit: Option<String>,
    is_permit_2: bool,
    taking_fee_receiver: Option<Address>,
    delay_auction_start_time_by: Option<u64>,
    order_expiration_delay: Option<u64>,
}

pub fn create_order(
    quote_request: QuoteRequest,
    quote_result: QuoteResult,
    params: CrossChainOrderParamsData,
) {
    let preset = params
        .preset
        .and_then(|preset| quote_result.get_preset(preset))
        .unwrap_or_else(|| quote_result.recommended_preset());

    let auction_details = preset.create_auction_details(params.delay_auction_start_time_by);

    let allow_partial_fills = preset.allow_partial_fills;
    let allow_multiple_fills = preset.allow_multiple_fills;
    let is_nonce_required = !allow_partial_fills || !allow_multiple_fills;

    let nonce = if is_nonce_required {
        params.nonce.or_else(|| {
            let mut rng = rand::rng();
            Some(rng.random_range(0..UINT_40_MAX))
        })
    } else {
        params.nonce
    };

    let taker_asset = quote_request.dst_token_address;
}

pub struct CrossChainOrder {
    escrow_extension: EscrowExtension,
    inner: FusionOrder,
}

pub struct EscrowParams {
    hash_lock: HashLock,
    src_chain_id: ChainId,
    dst_chain_id: ChainId,
    src_safety_deposit: U256,
    dst_safety_deposit: U256,
    timelocks: TimeLocks,
}

pub struct EscrowExtension {
    escrow_factory: Address,
    auction_details: AuctionDetails,
    post_interaction_data: SettlementPostInteractionData,
    maker_permit: Option<Interaction>,
    hash_lock_info: HashLock,
    dst_chain_id: ChainId,
    dst_token: Address,
    src_safety_deposit: U256,
    dst_safety_deposit: U256,
    time_locks: TimeLocks,
}

impl EscrowExtension {
    pub fn new(
        escrow_factory: Address,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        maker_permit: Option<Interaction>,
        hash_lock_info: HashLock,
        dst_chain_id: ChainId,
        mut dst_token: Address,
        src_safety_deposit: U256,
        dst_safety_deposit: U256,
        time_locks: TimeLocks,
    ) -> Self {
        assert!(src_safety_deposit <= UINT_128_MAX);
        assert!(dst_safety_deposit <= UINT_128_MAX);

        // TODO call construstor of FusionExtension

        if dst_token == Address::ZERO {
            dst_token = NATIVE_CURRENCY;
        }

        Self {
            escrow_factory,
            auction_details,
            post_interaction_data,
            maker_permit,
            hash_lock_info,
            dst_chain_id,
            dst_token,
            src_safety_deposit,
            dst_safety_deposit,
            time_locks,
        }
    }
}

pub struct DetailsFees {
    integrator_fee: IntegratorFee,
    bank_fee: Option<u64>,
}

pub struct Details {
    auction: AuctionDetails,
    fees: Option<DetailsFees>,
    whitelist: Vec<AuctionWhitelistItem>,
    resolving_start_time: Option<u64>,
}

pub struct Extra {
    nonce: Option<u64>,
    permit: Option<Bytes>,
    // Order will expire in `orderExpirationDelay` after auction ends Default 12s
    order_expiration_delay: Option<u64>,
    enable_permit2: Option<bool>,
    source: Option<String>,
    allow_multiple_fills: Option<bool>,
    allow_partial_fills: Option<bool>,
}

impl CrossChainOrder {
    pub fn new(
        src_escrow_factory: Address,
        order_info: OrderInfoData, // CrossChainOrderInfo,
        escrow_params: EscrowParams,
        details: Details,
        extra: Extra,
    ) -> Self {
        let post_interaction_data = SettlementPostInteractionData::new(SettlementSuffixData {
            whitelist: details.whitelist,
            integrator_fee: details.fees.as_ref().map(|f| f.integrator_fee.clone()),
            bank_fee: details.fees.as_ref().and_then(|f| f.bank_fee),
            resolving_start_time: details
                .resolving_start_time
                .unwrap_or_else(|| Utc::now().timestamp() as u64),
            custom_receiver: order_info.receiver,
        });

        let ext = EscrowExtension::new(
            src_escrow_factory,
            details.auction,
            post_interaction_data,
            extra.permit.map(|permit| Interaction {
                target: order_info.maker_asset,
                data: permit,
            }),
            escrow_params.hash_lock,
            escrow_params.dst_chain_id,
            order_info.taker_asset,
            escrow_params.src_safety_deposit,
            escrow_params.dst_safety_deposit,
            escrow_params.timelocks,
        );

        assert!(
            escrow_params.src_chain_id != escrow_params.dst_chain_id,
            "src and dst chain ids must be different"
        );

        Self::new_internal(ext, order_info, Some(extra))
    }

    fn new_internal(
        extension: EscrowExtension,
        order_info: OrderInfoData,
        extra: Option<Extra>,
    ) -> Self {
    }
}
