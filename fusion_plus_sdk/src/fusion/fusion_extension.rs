use alloy::primitives::Address;

use crate::fusion::{
    auction_details::AuctionDetails,
    fusion_order::{Fees, Interaction},
    settlement_post_interaction::SettlementPostInteractionData,
};

pub struct FusionExtension {
    pub settlement_extension_contract: Address,
    pub auction_details: AuctionDetails,
    pub post_interaction_data: SettlementPostInteractionData,
    pub maker_permit: Option<Interaction>,
}

// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/fusion-extension.ts#L19
pub struct FusionExtensionExtra {
    maker_permit: Interaction,
    custom_receiver: Option<Address>,
    fees: Fees,
}
