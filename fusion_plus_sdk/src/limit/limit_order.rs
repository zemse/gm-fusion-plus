use alloy::primitives::{B256, U256};

use crate::{
    chain_id::ChainId,
    constants::{NATIVE_CURRENCY, UINT_160_MAX},
    limit::{
        eip712::LimitOrderV4, extension::Extension, maker_traits::MakerTraits,
        order_info::OrderInfoData,
    },
    multichain_address::MultichainAddress,
    utils::{alloy::CustomAlloy, random::get_random_uint},
};

#[derive(Clone, Debug)]
pub struct LimitOrder {
    pub salt: U256,
    pub maker: MultichainAddress,
    pub receiver: MultichainAddress,
    pub maker_asset: MultichainAddress,
    pub taker_asset: MultichainAddress,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub maker_traits: MakerTraits,
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

        // https://github.com/1inch/limit-order-sdk/blob/e2c4d88ef3a830500dc957604958091039b32b96/src/limit-order/limit-order.ts#L69-L71
        let receiver = order_info
            .receiver
            .map(|receiver| {
                if receiver == order_info.maker {
                    MultichainAddress::ZERO
                } else {
                    receiver
                }
            })
            .unwrap_or(MultichainAddress::ZERO);

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
            receiver,
            maker_traits,
        }
    }

    pub fn to_v4(&self) -> LimitOrderV4 {
        LimitOrderV4 {
            salt: self.salt,
            maker: self.maker.as_raw(),
            receiver: self.receiver.as_raw(),
            makerAsset: self.maker_asset.as_raw(),
            takerAsset: self.taker_asset.as_raw(),
            makingAmount: self.making_amount,
            takingAmount: self.taking_amount,
            makerTraits: self.maker_traits.as_u256(),
        }
    }

    pub fn get_order_hash(&self, chain_id: ChainId) -> B256 {
        self.to_v4().get_order_hash(chain_id)
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

        salt
    }

    pub fn build_salt(extension: &Extension, base_salt: Option<U256>) -> U256 {
        let base_salt = base_salt.unwrap_or_else(get_random_uint::<12>);

        if extension.is_empty() {
            return base_salt;
        }

        (base_salt << 160) | (extension.keccak256().to_u256() & UINT_160_MAX)
    }
}
