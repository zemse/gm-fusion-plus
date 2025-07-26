use alloy::{
    dyn_abi::DynSolValue,
    primitives::{B256, keccak256},
};
use alloy_merkle_tree::standard_binary_tree::StandardMerkleTree;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct HashLock {
    pub hash: B256,
}

impl Serialize for HashLock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.hash.as_ref())
    }
}

impl HashLock {
    pub fn new(hash: B256) -> Self {
        HashLock { hash }
    }

    pub fn hash_secret(secret: &B256) -> B256 {
        keccak256(secret)
    }

    pub fn for_single_fill(secret: &B256) -> Self {
        let hash = Self::hash_secret(secret);
        HashLock::new(hash)
    }

    pub fn for_multiple_fills(secret: Vec<B256>) -> crate::Result<Self> {
        if secret.len() <= 2 {
            return Err(crate::Error::InternalErrorStr(
                "leaves array must be greater than 2. Or use HashLock.forSingleFill",
            ));
        }

        let root = StandardMerkleTree::of(
            // TODO PR to improve this code https://github.com/alloy-rs/core/pull/983
            &secret
                .iter()
                .map(|s| DynSolValue::FixedBytes(*s, 32))
                .collect::<Vec<DynSolValue>>(),
        )
        .root();

        let mut root_with_count = root;

        // setMask https://github.com/1inch/cross-chain-sdk/blob/25ac3927c706a43e85f2f08cc9d9a3bdf156e1e9/src/cross-chain-order/hash-lock/hash-lock.ts#L68
        let length = secret.len() as u16;
        root_with_count.0[0] = (length >> 8) as u8;
        root_with_count.0[1] = (length & 0xFF) as u8;

        Ok(HashLock::new(root_with_count))
    }
}
