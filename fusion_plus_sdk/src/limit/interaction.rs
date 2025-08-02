use alloy::primitives::Bytes;

use crate::{
    multichain_address::MultichainAddress,
    utils::bytes_iter::{BytesIter, Side},
};

#[cfg_attr(test, derive(Default, PartialEq))]
#[derive(Clone, Debug)]
pub struct Interaction {
    pub target: MultichainAddress,
    pub data: Bytes,
}

impl Interaction {
    pub fn encode(&self) -> Bytes {
        [self.target.as_raw().to_vec(), self.data.to_vec()]
            .concat()
            .into()
    }

    pub fn decode_from(bytes: Bytes) -> Self {
        let mut iter = BytesIter::new(bytes);

        let target = iter.next_address(Side::Front);
        let data = iter.rest();

        Self {
            target: MultichainAddress::from_raw(target),
            data,
        }
    }
}
