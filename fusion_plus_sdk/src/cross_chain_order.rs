use alloy::primitives::{Address, B256, Bytes};
use chrono::Utc;
use rand::Rng;
use serde::Serialize;

use crate::{
    chain_id::ChainId,
    constants::UINT_40_MAX,
    escrow_extension::{EscrowExtension, EscrowParams},
    fusion::{
        auction_details::{AuctionDetails, AuctionWhitelistItem},
        fusion_order::{FusionOrder, FusionOrderExtra, IntegratorFee},
        settlement_post_interaction::{SettlementPostInteractionData, SettlementSuffixData},
    },
    hash_lock::HashLock,
    limit::{interaction::Interaction, order_info::OrderInfoData},
    quote::{QuoteRequest, QuoteResult, preset::PresetType},
    utils::bps::Bps,
};

#[derive(Clone, Debug)]
pub struct PreparedOrder {
    pub order: CrossChainOrder,
    pub hash: B256,
    pub quote_id: String,
}

impl PreparedOrder {
    pub fn from_quote(
        quote_request: &QuoteRequest,
        quote_result: &QuoteResult,
        order_params: CrossChainOrderParams,
    ) -> crate::Result<PreparedOrder> {
        let Some(quote_id) = &quote_result.quote_id else {
            return Err(crate::Error::InternalErrorStr(
                "request quote with enableEstimate=true",
            ));
        };

        let order = CrossChainOrder::from_quote(
            quote_request,
            quote_result,
            CrossChainOrderParamsData {
                hash_lock: order_params.hash_lock,
                preset: None, // PresetType::Fast,
                receiver: Some(order_params.dst_address),
                nonce: None, // Some(0),
                permit: None,
                is_permit_2: false,
                taking_fee_receiver: None,
                delay_auction_start_time_by: None,
                order_expiration_delay: None,
            },
        );

        let hash = order.get_order_hash(quote_request.src_chain_id);

        Ok(PreparedOrder {
            order,
            hash,
            quote_id: quote_id.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CrossChainOrderParams {
    #[serde(rename = "walletAddress")]
    pub dst_address: Address,

    #[serde(rename = "hashLock")]
    pub hash_lock: HashLock,

    pub secret_hashes: Vec<B256>,
}

pub struct Fee {
    pub taking_fee_bps: u16,
    pub taking_fee_receiver: Address,
}

// https://github.com/1inch/cross-chain-sdk/blob/25ac3927c706a43e85f2f08cc9d9a3bdf156e1e9/src/api/quoter/quote/types.ts#L5
pub struct CrossChainOrderParamsData {
    hash_lock: HashLock,
    preset: Option<PresetType>,
    receiver: Option<Address>,
    nonce: Option<u64>,
    permit: Option<Bytes>,
    is_permit_2: bool,
    taking_fee_receiver: Option<Address>,
    delay_auction_start_time_by: Option<u64>,
    order_expiration_delay: Option<u64>,
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

pub struct CrossChainExtra {
    nonce: Option<u64>,
    permit: Option<Bytes>,
    // Order will expire in `orderExpirationDelay` after auction ends Default 12s
    order_expiration_delay: Option<u64>,
    enable_permit2: Option<bool>,
    source: Option<String>,
    allow_multiple_fills: Option<bool>,
    allow_partial_fills: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct CrossChainOrder {
    inner: FusionOrder<EscrowExtension>,
}

impl CrossChainOrder {
    pub fn from_quote(
        quote_request: &QuoteRequest,
        quote_result: &QuoteResult,
        params: CrossChainOrderParamsData,
    ) -> CrossChainOrder {
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

        let whitelist = quote_result.get_whitelist(
            auction_details.start_time,
            preset.exclusive_resolver.as_ref(),
        );

        CrossChainOrder::new(
            quote_result.src_escrow_factory,
            OrderInfoData {
                maker_asset: quote_request.src_token_address,
                taker_asset,
                making_amount: quote_result.src_token_amount,
                taking_amount: quote_result.dst_token_amount,
                maker: quote_request.maker_address,
                receiver: params.receiver,
                salt: None,
            },
            EscrowParams {
                hash_lock: params.hash_lock,
                src_chain_id: quote_request.src_chain_id,
                dst_chain_id: quote_request.dst_chain_id,
                src_safety_deposit: quote_result.src_safety_deposit,
                dst_safety_deposit: quote_result.dst_safety_deposit,
                timelocks: quote_result.time_locks.clone(),
            },
            Details {
                auction: auction_details,
                fees: Some(DetailsFees {
                    integrator_fee: IntegratorFee {
                        receiver: params.taking_fee_receiver.unwrap_or_default(),
                        ratio: Bps::to_ratio_format(quote_request.fee) as u16,
                    },
                    bank_fee: Some(0),
                }),
                whitelist,
                resolving_start_time: None,
            },
            Some(CrossChainExtra {
                nonce,
                permit: params.permit,
                order_expiration_delay: params.order_expiration_delay,
                enable_permit2: Some(params.is_permit_2),
                source: quote_request.source.clone(),
                allow_multiple_fills: Some(allow_multiple_fills),
                allow_partial_fills: Some(allow_partial_fills),
            }),
        )
    }

    pub fn new(
        src_escrow_factory: Address,
        order_info: OrderInfoData, // CrossChainOrderInfo,
        escrow_params: EscrowParams,
        details: Details,
        extra: Option<CrossChainExtra>,
    ) -> Self {
        let post_interaction_data = SettlementPostInteractionData::new(SettlementSuffixData {
            bank_fee: details.fees.as_ref().and_then(|f| f.bank_fee),
            integrator_fee: details.fees.as_ref().map(|f| f.integrator_fee.clone()),
            whitelist: details.whitelist,
            resolving_start_time: details
                .resolving_start_time
                .unwrap_or_else(|| Utc::now().timestamp() as u64),
            custom_receiver: order_info.receiver,
        });

        assert!(
            escrow_params.src_chain_id != escrow_params.dst_chain_id,
            "src and dst chain ids must be different"
        );

        let ext = EscrowExtension::new(
            src_escrow_factory,
            details.auction,
            post_interaction_data,
            extra
                .as_ref()
                .and_then(|extra| extra.permit.as_ref())
                .map(|permit| Interaction {
                    target: order_info.maker_asset,
                    data: permit.clone(),
                }),
            order_info.taker_asset,
            escrow_params,
        );

        Self::new_from_extension(ext, order_info, extra)
    }

    fn new_from_extension(
        extension: EscrowExtension,
        order_info: OrderInfoData,
        extra: Option<CrossChainExtra>,
    ) -> Self {
        Self {
            inner: FusionOrder::new_with_extension(
                extension.fusion_extension.settlement_extension_contract,
                order_info,
                extension.fusion_extension.auction_details.clone(),
                extension.fusion_extension.post_interaction_data.clone(),
                extra.map(Into::into),
                extension,
            ),
        }
    }

    pub fn get_order_hash(&self, src_chain_id: ChainId) -> B256 {
        self.inner.get_order_hash(src_chain_id)
    }
}

impl From<CrossChainExtra> for FusionOrderExtra {
    fn from(extra: CrossChainExtra) -> Self {
        Self {
            unwrap_weth: None,
            nonce: extra.nonce,
            permit: extra.permit,
            allow_partial_fills: extra.allow_partial_fills,
            allow_multiple_fills: extra.allow_multiple_fills,
            order_expiration_delay: extra.order_expiration_delay,
            enable_permit2: extra.enable_permit2,
            source: extra.source,
            fees: None,
        }
    }
}
