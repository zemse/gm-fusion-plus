use alloy::primitives::{B256, Bytes, U256, keccak256};

use crate::{
    constants::UINT_32_MAX,
    utils::bytes_iter::{BytesIter, Side},
};

#[derive(Default)]
pub struct Extension {
    pub maker_asset_suffix: Bytes,
    pub taker_asset_suffix: Bytes,
    pub making_amount_data: Bytes,
    pub taking_amount_data: Bytes,
    pub predicate: Bytes,
    pub maker_permit: Bytes,
    pub pre_interaction: Bytes,
    pub post_interaction: Bytes,
    pub custom_data: Bytes,
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

    pub fn get_all_mut(&mut self) -> [&mut Bytes; 8] {
        [
            &mut self.maker_asset_suffix,
            &mut self.taker_asset_suffix,
            &mut self.making_amount_data,
            &mut self.taking_amount_data,
            &mut self.predicate,
            &mut self.maker_permit,
            &mut self.pre_interaction,
            &mut self.post_interaction,
        ]
    }

    pub fn append_post_interaction(mut self, bytes: Bytes) -> Self {
        self.post_interaction = [self.post_interaction.clone(), bytes].concat().into();
        self
    }

    pub fn with_post_interaction(mut self, bytes: Bytes) -> Self {
        self.post_interaction = bytes;
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

    pub fn decode_from(bytes: Bytes) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }

        let mut extension = Self::default();

        let mut iter = BytesIter::new(bytes);

        let mut offsets = iter.next_uint256(Side::Front);
        let mut consumed = 0;

        for data in extension.get_all_mut() {
            let offset = (offsets & U256::from(UINT_32_MAX)).to::<usize>();
            let bytes_count = offset - consumed;
            *data = iter.next_bytes(bytes_count, Side::Front);

            consumed += bytes_count;
            offsets >>= 32;
        }

        extension.custom_data = iter.rest();

        extension
    }
}

#[cfg(test)]
mod tests {
    use super::Extension;

    #[test]
    fn encode_decode_1() {
        let extension = Extension {
            maker_asset_suffix: "0x".parse().unwrap(),
            taker_asset_suffix: "0x".parse().unwrap(),
            making_amount_data: "0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a00ed9c000002206887cceb0000b4027a9501e371007800ed9c003c".parse().unwrap(),
            taking_amount_data: "0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a00ed9c000002206887cceb0000b4027a9501e371007800ed9c003c".parse().unwrap(),
            predicate: "0x".parse().unwrap(),
            maker_permit: "0x".parse().unwrap(),
            pre_interaction: "0x".parse().unwrap(),
            post_interaction: "0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a6887ccd3cb4fa6eb00f6ea887a4a000072f8a0c8c415454f629c00005ba74b09ae44e823cf77000018a0317c640f8287b517e160731e0dbfd09ce2a0f5040836818a0f547b0c4da964000000000000000000000000000000000000000000000000000000000000a4b1000000000000000000000000af88d065e77c8cc2239327c5edb3a432268e58310000000000000000000067e7a63e314000000000000000000000046a392bdb6000000000000001c8000001500000003c00000288000002100000017400000024".parse().unwrap(),
            custom_data: "0x".parse().unwrap(),
        };

        let encoded = extension.encode();
        let decoded = Extension::decode_from(encoded);

        assert_eq!(extension.maker_asset_suffix, decoded.maker_asset_suffix);
        assert_eq!(extension.taker_asset_suffix, decoded.taker_asset_suffix);
        assert_eq!(extension.making_amount_data, decoded.making_amount_data);
        assert_eq!(extension.taking_amount_data, decoded.taking_amount_data);
        assert_eq!(extension.predicate, decoded.predicate);
        assert_eq!(extension.maker_permit, decoded.maker_permit);
        assert_eq!(extension.pre_interaction, decoded.pre_interaction);
        assert_eq!(extension.post_interaction, decoded.post_interaction);
        assert_eq!(extension.custom_data, decoded.custom_data);
    }
}
