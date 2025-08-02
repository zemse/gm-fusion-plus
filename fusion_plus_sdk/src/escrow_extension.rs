use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    primitives::{Address, Bytes, U256},
};

use crate::{
    chain_id::ChainId,
    constants::{NATIVE_CURRENCY, UINT_128_MAX},
    fusion::{
        auction_details::AuctionDetails, fusion_extension::FusionExtension,
        settlement_post_interaction::SettlementPostInteractionData,
    },
    hash_lock::HashLock,
    limit::{
        extension::Extension, extension_builder::ExtensionBuildable, interaction::Interaction,
    },
    multichain_address::MultichainAddress,
    time_locks::TimeLocks,
};

const EXTRA_DATA_BYTES_LENGTH: usize = 160;

pub struct EscrowParams {
    pub hash_lock: HashLock,
    pub src_chain_id: ChainId,
    pub dst_chain_id: ChainId,
    pub src_safety_deposit: U256,
    pub dst_safety_deposit: U256,
    pub timelocks: TimeLocks,
}

#[derive(Clone, Debug)]
pub struct EscrowExtension {
    pub fusion_extension: FusionExtension,
    pub hash_lock_info: HashLock,
    pub dst_chain_id: ChainId,
    pub dst_token: MultichainAddress,
    pub src_safety_deposit: U256,
    pub dst_safety_deposit: U256,
    pub time_locks: TimeLocks,
}

impl EscrowExtension {
    pub fn new(
        escrow_factory: MultichainAddress,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        maker_permit: Option<Interaction>,
        mut dst_token: MultichainAddress,
        escrow_params: EscrowParams,
    ) -> Self {
        assert!(escrow_params.src_safety_deposit <= UINT_128_MAX);
        assert!(escrow_params.dst_safety_deposit <= UINT_128_MAX);

        let fusion_extension = FusionExtension::new(
            escrow_factory,
            auction_details,
            post_interaction_data,
            maker_permit,
        );

        if dst_token == MultichainAddress::ZERO {
            dst_token = NATIVE_CURRENCY;
        }

        Self {
            fusion_extension,
            hash_lock_info: escrow_params.hash_lock,
            dst_chain_id: escrow_params.dst_chain_id,
            dst_token,
            src_safety_deposit: escrow_params.src_safety_deposit,
            dst_safety_deposit: escrow_params.dst_safety_deposit,
            time_locks: escrow_params.timelocks,
        }
    }

    pub fn encode_extra_data(&self) -> Bytes {
        let dst_token = if self.dst_token == NATIVE_CURRENCY {
            MultichainAddress::ZERO
        } else {
            self.dst_token
        };

        DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(self.hash_lock_info.value(), 32),
            DynSolValue::Uint(U256::from(self.dst_chain_id as u64), 256),
            DynSolValue::Address(dst_token.as_raw()),
            DynSolValue::Uint(
                (self.src_safety_deposit << 128) | self.dst_safety_deposit,
                256,
            ),
            DynSolValue::Uint(self.time_locks.build(), 256),
        ])
        .abi_encode()
        .into()
    }

    pub fn decode_from(bytes: Bytes) -> Self {
        let extension = Extension::decode_from(bytes);
        EscrowExtension::from_extension(extension)
    }

    pub fn from_extension(extension: Extension) -> Self {
        let (base_post_interaction, extra_data) = {
            let post_interaction = extension.post_interaction.clone();
            let extra_data_start = post_interaction
                .len()
                .checked_sub(EXTRA_DATA_BYTES_LENGTH)
                .expect("post interaction data too short for escrow extra data");
            let (base, tail) = post_interaction.split_at(extra_data_start);
            (base.to_vec().into(), tail.to_vec().into())
        };

        let base_extension = extension.with_post_interaction(base_post_interaction);
        let fusion_ext = FusionExtension::from_extension(base_extension);

        let (
            hash_lock,
            dst_chain_id,
            dst_token,
            src_safety_deposit,
            dst_safety_deposit,
            time_locks,
        ) = EscrowExtension::decode_extra_data(extra_data);

        EscrowExtension {
            fusion_extension: fusion_ext,
            hash_lock_info: hash_lock,
            dst_chain_id,
            dst_token,
            src_safety_deposit,
            dst_safety_deposit,
            time_locks,
        }
    }

    pub fn decode_extra_data(
        bytes: Bytes,
    ) -> (HashLock, ChainId, MultichainAddress, U256, U256, TimeLocks) {
        let schema = DynSolType::Tuple(vec![
            DynSolType::FixedBytes(32), // hash_lock
            DynSolType::Uint(256),      // dst_chain_id
            DynSolType::Address,        // dst_token
            DynSolType::Uint(256),      // safety_deposit (128+128)
            DynSolType::Uint(256),      // time_locks
        ]);

        let DynSolValue::Tuple(values) = schema
            .abi_decode(&bytes)
            .expect("Invalid extra data encoding")
        else {
            panic!("Expected tuple decoding");
        };

        let hash_lock = values[0].as_word().expect("Invalid hash_lock type");

        let (dst_chain_id, _) = values[1].as_uint().expect("Invalid dst_chain_id type");
        let dst_chain_id = ChainId::from_u32(dst_chain_id.to::<u32>());

        let dst_token = match values[2].as_address().expect("Invalid dst_token type") {
            Address::ZERO => NATIVE_CURRENCY,
            addr => MultichainAddress::from_raw(addr),
        };

        let (src_safety_deposit, dst_safety_deposit) = {
            let (safety, _) = values[3].as_uint().expect("Invalid safety deposit type");
            let src = safety >> 128;
            let dst = safety & UINT_128_MAX;
            (src, dst)
        };

        let (time_locks, _) = values[4].as_uint().expect("Invalid time locks type");

        (
            HashLock::new(hash_lock),
            dst_chain_id,
            dst_token,
            src_safety_deposit,
            dst_safety_deposit,
            TimeLocks::from_u256(time_locks),
        )
    }
}

impl ExtensionBuildable for EscrowExtension {
    fn build(&self) -> Extension {
        self.fusion_extension
            .build()
            .append_post_interaction(self.encode_extra_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_fees_encode_decode() {
        let ext = EscrowExtension {
            fusion_extension: FusionExtension::default(),
            hash_lock_info: HashLock::new([3; 32].into()),
            dst_chain_id: ChainId::Ethereum,
            dst_token: MultichainAddress::from_raw(Address::ZERO.create(1)),
            src_safety_deposit: U256::from(1000),
            dst_safety_deposit: U256::from(2000),
            time_locks: TimeLocks::new(36, 372, 528, 648, 60, 336, 456, Some(80)),
        };

        let encoded = ext.encode_extra_data();
        let decoded = EscrowExtension::decode_extra_data(encoded);

        assert_eq!(decoded.0, ext.hash_lock_info);
        assert_eq!(decoded.1, ext.dst_chain_id);
        assert_eq!(decoded.2, ext.dst_token);
        assert_eq!(decoded.3, ext.src_safety_deposit);
        assert_eq!(decoded.4, ext.dst_safety_deposit);
        assert_eq!(decoded.5, ext.time_locks);
    }
}
