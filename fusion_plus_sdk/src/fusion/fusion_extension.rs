use crate::{
    fusion::{
        auction_details::AuctionDetails, fusion_order::Fees,
        settlement_post_interaction::SettlementPostInteractionData,
    },
    limit::{
        extension::Extension,
        extension_builder::{ExtensionBuildable, ExtensionBuilder},
        interaction::Interaction,
    },
    multichain_address::MultichainAddress,
    utils::bytes_iter::BytesIter,
};

#[cfg_attr(test, derive(Default, PartialEq))]
#[derive(Clone, Debug)]
pub struct FusionExtension {
    pub settlement_extension_contract: MultichainAddress,
    pub auction_details: AuctionDetails,
    pub post_interaction_data: SettlementPostInteractionData,
    pub maker_permit: Option<Interaction>,
}

impl FusionExtension {
    pub fn new(
        settlement_extension_contract: MultichainAddress,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        maker_permit: Option<Interaction>,
    ) -> Self {
        Self {
            settlement_extension_contract,
            auction_details,
            post_interaction_data,
            maker_permit,
        }
    }

    pub fn from_extension(extension: Extension) -> Self {
        let settlement_contract_1 = BytesIter::first_address(extension.making_amount_data.clone());
        let settlement_contract_2 = BytesIter::first_address(extension.taking_amount_data.clone());
        let settlement_extension_3 = BytesIter::first_address(extension.post_interaction.clone());

        assert!(
            settlement_contract_1 == settlement_contract_2
                && settlement_contract_1 == settlement_extension_3,
            "Invalid extension, all calls should be to the same address"
        );

        // TODO this uses making_amount_data only. There seems to be no place that uses taking_amount_data
        let auction_details = AuctionDetails::from_extension(&extension);

        let post_interaction_data = SettlementPostInteractionData::from_extension(&extension);

        let maker_permit = if extension.maker_permit.is_empty() {
            None
        } else {
            Some(Interaction::decode_from(extension.maker_permit))
        };

        Self {
            settlement_extension_contract: MultichainAddress::from_raw(settlement_contract_1),
            auction_details,
            post_interaction_data,
            maker_permit,
        }
    }
}

impl ExtensionBuildable for FusionExtension {
    fn build(&self) -> Extension {
        let mut builder = ExtensionBuilder::default();

        let details_bytes = self.auction_details.encode();
        builder = builder
            .with_making_amount_data(self.settlement_extension_contract, details_bytes.clone())
            .with_taking_amount_data(self.settlement_extension_contract, details_bytes)
            .with_post_interaction(Interaction {
                target: self.settlement_extension_contract,
                data: self.post_interaction_data.encode(),
            });

        if let Some(maker_permit) = &self.maker_permit {
            builder = builder.with_maker_permit(maker_permit.target, &maker_permit.data);
        }

        builder.build()
    }
}

// TODO seems this is not used anywhere
// https://github.com/1inch/fusion-sdk/blob/6d40f680a2f1cd0148c314d4c8608a004fffdc09/src/fusion-order/fusion-extension.ts#L19
#[allow(dead_code)]
pub struct FusionExtensionExtra {
    maker_permit: Interaction,
    custom_receiver: Option<MultichainAddress>,
    fees: Fees,
}
