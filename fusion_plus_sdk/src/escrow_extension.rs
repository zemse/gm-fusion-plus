use alloy::{
    dyn_abi::DynSolValue,
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
        extension::{Extension, ExtensionBuildable},
        interaction::Interaction,
    },
    time_locks::TimeLocks,
};

#[derive(Clone, Debug)]
pub struct EscrowExtension {
    pub fusion_extension: FusionExtension,
    pub hash_lock_info: HashLock,
    pub dst_chain_id: ChainId,
    pub dst_token: Address,
    pub src_safety_deposit: U256,
    pub dst_safety_deposit: U256,
    pub time_locks: TimeLocks,
}

impl EscrowExtension {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        escrow_factory: Address,
        auction_details: AuctionDetails,
        post_interaction_data: SettlementPostInteractionData,
        maker_permit: Option<Interaction>,
        hash_lock_info: HashLock,
        dst_chain_id: ChainId,
        mut dst_token: Address,
        src_safety_deposit: U256,
        dst_safety_deposit: U256,
        time_locks: TimeLocks,
    ) -> Self {
        assert!(src_safety_deposit <= UINT_128_MAX);
        assert!(dst_safety_deposit <= UINT_128_MAX);

        let fusion_extension = FusionExtension::new(
            escrow_factory,
            auction_details,
            post_interaction_data,
            maker_permit,
        );

        if dst_token == Address::ZERO {
            dst_token = NATIVE_CURRENCY;
        }

        Self {
            fusion_extension,
            hash_lock_info,
            dst_chain_id,
            dst_token,
            src_safety_deposit,
            dst_safety_deposit,
            time_locks,
        }
    }

    pub fn encode_extra_data(&self) -> Bytes {
        let dst_token = if self.dst_token == NATIVE_CURRENCY {
            Address::ZERO
        } else {
            self.dst_token
        };

        DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(self.hash_lock_info.value(), 32),
            DynSolValue::Uint(U256::from(self.dst_chain_id as u64), 256),
            DynSolValue::Address(dst_token),
            DynSolValue::Uint(
                (self.src_safety_deposit << 128) | self.dst_safety_deposit,
                256,
            ),
            DynSolValue::Uint(self.time_locks.build(), 256),
        ])
        .abi_encode()
        .into()
    }
}

impl ExtensionBuildable for EscrowExtension {
    fn build(&self) -> Extension {
        self.fusion_extension
            .build()
            .append_post_interaction(self.encode_extra_data())
    }
}
