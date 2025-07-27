use alloy::primitives::Address;

use crate::{
    fusion::{
        auction_details::AuctionDetails, fusion_order::Fees,
        settlement_post_interaction::SettlementPostInteractionData,
    },
    limit::{
        extension::{Extension, ExtensionBuildable, ExtensionBuilder},
        interaction::Interaction,
    },
};

#[derive(Clone, Debug)]
pub struct FusionExtension {
    builder: ExtensionBuilder,
    pub settlement_extension_contract: Address,
    pub auction_details: AuctionDetails,
    pub post_interaction_data: SettlementPostInteractionData,
    pub maker_permit: Option<Interaction>,
}

impl FusionExtension {
    pub fn new(
        settlement_extension_contract: Address,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        maker_permit: Option<Interaction>,
    ) -> Self {
        let mut builder = ExtensionBuilder::default();

        let details_bytes = auction_details.encode();
        builder = builder
            .with_making_amount_data(settlement_extension_contract, details_bytes.clone())
            .with_taking_amount_data(settlement_extension_contract, details_bytes)
            .with_post_interaction(Interaction {
                target: settlement_extension_contract,
                data: post_interaction_data.encode(),
            });

        if let Some(maker_permit) = &maker_permit {
            builder = builder.with_maker_permit(maker_permit.target, &maker_permit.data);
        }

        Self {
            builder,
            settlement_extension_contract,
            auction_details,
            post_interaction_data,
            maker_permit,
        }
    }
}

impl ExtensionBuildable for FusionExtension {
    fn build(&self) -> Extension {
        self.builder.clone().build()
    }
}

// TODO seems this is not used anywhere
// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/fusion-extension.ts#L19
pub struct FusionExtensionExtra {
    maker_permit: Interaction,
    custom_receiver: Option<Address>,
    fees: Fees,
}
