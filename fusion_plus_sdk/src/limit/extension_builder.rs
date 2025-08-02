use std::fmt::Debug;

use alloy::primitives::Bytes;

use crate::{
    limit::{extension::Extension, interaction::Interaction},
    multichain_address::MultichainAddress,
};

pub trait ExtensionBuildable: Clone + Debug {
    fn build(&self) -> Extension;
}

#[derive(Clone, Debug, Default)]
pub struct ExtensionBuilder {
    maker_asset_suffix: Bytes,
    taker_asset_suffix: Bytes,
    making_amount_data: Bytes,
    taking_amount_data: Bytes,
    predicate: Bytes,
    maker_permit: Bytes,
    pre_interaction: Bytes,
    post_interaction: Bytes,
    custom_data: Bytes,
}

impl ExtensionBuilder {
    pub fn with_maker_asset_suffix(mut self, suffix: Bytes) -> Self {
        self.maker_asset_suffix = suffix;
        self
    }

    pub fn with_taker_asset_suffix(mut self, suffix: Bytes) -> Self {
        self.taker_asset_suffix = suffix;
        self
    }

    pub fn with_making_amount_data(mut self, address: MultichainAddress, data: Bytes) -> Self {
        self.making_amount_data = [address.as_raw().to_vec(), data.to_vec()].concat().into();
        self
    }

    pub fn with_taking_amount_data(mut self, address: MultichainAddress, data: Bytes) -> Self {
        self.taking_amount_data = [address.as_raw().to_vec(), data.to_vec()].concat().into();
        self
    }

    pub fn with_predicate(mut self, predicate: Bytes) -> Self {
        self.predicate = predicate;
        self
    }

    pub fn with_maker_permit(mut self, token_from: MultichainAddress, permit_data: &Bytes) -> Self {
        self.maker_permit = [token_from.as_raw().to_vec(), permit_data.to_vec()]
            .concat()
            .into();
        self
    }

    pub fn with_pre_interaction(mut self, interaction: Interaction) -> Self {
        self.pre_interaction = interaction.encode();
        self
    }

    pub fn with_post_interaction(mut self, interaction: Interaction) -> Self {
        self.post_interaction = interaction.encode();
        self
    }

    pub fn with_custom_data(mut self, data: Bytes) -> Self {
        self.custom_data = data;
        self
    }
}

impl ExtensionBuildable for ExtensionBuilder {
    fn build(&self) -> Extension {
        Extension {
            maker_asset_suffix: self.maker_asset_suffix.clone(),
            taker_asset_suffix: self.taker_asset_suffix.clone(),
            making_amount_data: self.making_amount_data.clone(),
            taking_amount_data: self.taking_amount_data.clone(),
            predicate: self.predicate.clone(),
            maker_permit: self.maker_permit.clone(),
            pre_interaction: self.pre_interaction.clone(),
            post_interaction: self.post_interaction.clone(),
            custom_data: self.custom_data.clone(),
        }
    }
}
