use alloy::primitives::{Address, B256, U256};

use crate::{
    chain_id::ChainId,
    constants::{NATIVE_CURRENCY, UINT_160_MAX},
    limit::{
        eip712::{self, get_limit_order_v4_domain, get_order_hash},
        extension::Extension,
        maker_traits::MakerTraits,
        order_info::OrderInfoData,
    },
    utils::{alloy::CustomAlloy, random::get_random_uint},
};

#[derive(Clone, Debug)]
pub struct LimitOrder {
    salt: U256,
    maker: Address,
    receiver: Address,
    maker_asset: Address,
    taker_asset: Address,
    making_amount: U256,
    taking_amount: U256,
    maker_traits: MakerTraits,
}

impl LimitOrder {
    pub fn new(
        order_info: OrderInfoData,
        maker_traits: Option<MakerTraits>,
        extension: Option<Extension>,
    ) -> Self {
        let mut maker_traits = maker_traits.unwrap_or_default();
        let extension = extension.unwrap_or_default();

        assert!(
            order_info.taker_asset != NATIVE_CURRENCY,
            "{:?} can not be 'takerAsset'. Use wrapper address as 'takerAsset' and 'makerTraits.enableNativeUnwrap' to swap to NATIVE currency",
            order_info.taker_asset
        );
        assert!(
            order_info.maker_asset != NATIVE_CURRENCY,
            "Maker asset {:?} can not be NATIVE, use wrapper",
            order_info.maker_asset
        );

        let salt = LimitOrder::verify_salt(
            order_info
                .salt
                .unwrap_or_else(|| LimitOrder::build_salt(&extension, None)),
            &extension,
        );
        // let built_extension = extension.build();

        if !extension.is_empty() {
            maker_traits = maker_traits.with_extension();
        }

        Self {
            maker_asset: order_info.maker_asset,
            taker_asset: order_info.taker_asset,
            making_amount: order_info.making_amount,
            taking_amount: order_info.taking_amount,
            salt,
            maker: order_info.maker,
            receiver: order_info.receiver.unwrap_or_default(),

            maker_traits,
        }
    }

    pub fn get_order_hash(&self, chain_id: ChainId) -> B256 {
        let domain = get_limit_order_v4_domain(chain_id);

        get_order_hash(
            &eip712::Order {
                salt: self.salt,
                maker: self.maker,
                receiver: self.receiver,
                makerAsset: self.maker_asset,
                takerAsset: self.taker_asset,
                makingAmount: self.making_amount,
                takingAmount: self.taking_amount,
                makerTraits: self.maker_traits.as_u256(),
            },
            &domain,
        )
    }

    pub fn verify_salt(salt: U256, extension: &Extension) -> U256 {
        if extension.is_empty() {
            return salt;
        }

        let hash = salt & UINT_160_MAX;
        let expected_hash = extension.keccak256().to_u256() & UINT_160_MAX;

        assert_eq!(
            hash, expected_hash,
            "invalid salt: lowest 160 bits should be extension hash"
        );

        hash
    }

    pub fn build_salt(extension: &Extension, base_salt: Option<U256>) -> U256 {
        let base_salt = base_salt.unwrap_or_else(|| get_random_uint::<12>());

        if extension.is_empty() {
            return base_salt;
        }

        (base_salt << 160) | (extension.keccak256().to_u256() & UINT_160_MAX)
    }
}
