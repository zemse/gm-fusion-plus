use std::fmt::Debug;

use alloy::primitives::{Address, B256, Bytes, U256, keccak256};

use crate::limit::interaction::Interaction;

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

    pub fn with_making_amount_data(mut self, address: Address, data: Bytes) -> Self {
        self.making_amount_data = [address.to_vec(), data.to_vec()].concat().into();
        self
    }

    pub fn with_taking_amount_data(mut self, address: Address, data: Bytes) -> Self {
        self.taking_amount_data = [address.to_vec(), data.to_vec()].concat().into();
        self
    }

    pub fn with_predicate(mut self, predicate: Bytes) -> Self {
        self.predicate = predicate;
        self
    }

    pub fn with_maker_permit(mut self, token_from: Address, permit_data: &Bytes) -> Self {
        self.maker_permit = [token_from.to_vec(), permit_data.to_vec()].concat().into();
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

    pub fn build(self) -> Extension {
        Extension {
            maker_asset_suffix: self.maker_asset_suffix,
            taker_asset_suffix: self.taker_asset_suffix,
            making_amount_data: self.making_amount_data,
            taking_amount_data: self.taking_amount_data,
            predicate: self.predicate,
            maker_permit: self.maker_permit,
            pre_interaction: self.pre_interaction,
            post_interaction: self.post_interaction,
            custom_data: self.custom_data,
        }
    }
}

#[derive(Default)]
pub struct Extension {
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

impl Extension {
    pub fn is_empty(&self) -> bool {
        let is_empty = self
            .get_all()
            .iter()
            .all(|interaction| interaction.is_empty());

        is_empty && self.custom_data.is_empty()
    }

    pub fn keccak256(&self) -> B256 {
        keccak256(self.encode())
    }

    pub fn get_all(&self) -> [&Bytes; 8] {
        [
            &self.maker_asset_suffix,
            &self.taker_asset_suffix,
            &self.making_amount_data,
            &self.taking_amount_data,
            &self.predicate,
            &self.maker_permit,
            &self.pre_interaction,
            &self.post_interaction,
        ]
    }

    pub fn append_post_interaction(mut self, bytes: Bytes) -> Self {
        self.post_interaction = [self.post_interaction.clone(), bytes].concat().into();
        self
    }

    pub fn encode(&self) -> Bytes {
        if self.is_empty() {
            Bytes::new()
        } else {
            let all_interactions = self.get_all();
            let offsets = {
                let mut value = U256::ZERO;
                let mut sum = 0;
                for (i, interaction) in all_interactions.iter().enumerate() {
                    sum += interaction.len();

                    value |= U256::from(sum) << (i * 32);
                }
                value
            };

            let concat = all_interactions
                .into_iter()
                .fold(Bytes::new(), |acc, interaction| {
                    [acc, interaction.clone()].concat().into()
                });

            [
                offsets.to_be_bytes::<32>().into(),
                concat,
                self.custom_data.clone(),
            ]
            .concat()
            .into()
        }
    }
}
