use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Unexpected},
};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChainId {
    Ethereum = 1,
    Optimism = 10,
    Arbitrum = 42161,
}

impl ChainId {
    pub fn try_from_u32(n: u32) -> crate::Result<Self> {
        match n {
            1 => Ok(ChainId::Ethereum),
            42161 => Ok(ChainId::Arbitrum),
            _ => Err(crate::Error::UnsupportedChainId(n)),
        }
    }

    pub fn from_u32(n: u32) -> Self {
        ChainId::try_from_u32(n).unwrap()
    }

    pub fn from_network_name(name: &str) -> crate::Result<Self> {
        match name {
            "ethereum" | "eth" => Ok(ChainId::Ethereum),
            "optimism" | "op" => Ok(ChainId::Optimism),
            "arbitrum" | "arb" => Ok(ChainId::Optimism),
            _ => Err(crate::Error::NetworkNameNotRecognised(name.to_string())),
        }
    }

    pub fn to_network_name(&self) -> String {
        match self {
            ChainId::Ethereum => "eth",
            ChainId::Optimism => "op",
            ChainId::Arbitrum => "arb",
        }
        .to_string()
    }
}

impl From<ChainId> for u32 {
    fn from(val: ChainId) -> Self {
        val as u32
    }
}

impl Serialize for ChainId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

impl<'de> Deserialize<'de> for ChainId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u32::deserialize(deserializer)?;
        ChainId::try_from_u32(v)
            .map_err(|e| D::Error::invalid_value(Unexpected::Str(&e.to_string()), &"valid ChainId"))
    }
}
