pub enum ChainId {
    Ethereum = 1,
    Arbitrum = 42161,
}

impl From<ChainId> for u32 {
    fn from(val: ChainId) -> Self {
        val as u32
    }
}
