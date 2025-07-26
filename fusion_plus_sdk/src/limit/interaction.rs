use alloy::primitives::{Address, Bytes};

#[derive(Clone, Debug)]
pub struct Interaction {
    pub target: Address,
    pub data: Bytes,
}

impl Interaction {
    pub fn encode(&self) -> Bytes {
        [self.target.to_vec(), self.data.to_vec()].concat().into()
    }
}
