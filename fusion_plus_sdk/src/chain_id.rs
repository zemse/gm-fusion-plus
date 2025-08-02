use std::{fmt::Display, str::FromStr};

use num_enum::TryFromPrimitive;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Unexpected},
};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
pub enum ChainId {
    Ethereum = 1,
    Optimism = 10,
    Arbitrum = 42161,
    Tron = 728126428,
}

impl Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_network_name())
    }
}

impl ChainId {
    pub fn from_u32(n: u32) -> Self {
        ChainId::try_from_primitive(n).unwrap()
    }

    pub fn from_network_name(name: &str) -> crate::Result<Self> {
        match name {
            "ethereum" | "eth" => Ok(ChainId::Ethereum),
            "optimism" | "op" => Ok(ChainId::Optimism),
            "arbitrum" | "arb" => Ok(ChainId::Arbitrum),
            "tron" => Ok(ChainId::Tron),
            _ => Err(crate::Error::NetworkNameNotRecognised(name.to_string())),
        }
    }

    pub fn to_network_name(&self) -> String {
        match self {
            ChainId::Ethereum => "eth",
            ChainId::Optimism => "op",
            ChainId::Arbitrum => "arb",
            ChainId::Tron => "tron",
        }
        .to_string()
    }
}

impl FromStr for ChainId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(result) = Self::from_network_name(s) {
            Ok(result)
        } else if let Ok(number) = u32::from_str(s) {
            ChainId::try_from_primitive(number)
                .map_err(|e| crate::Error::UnsupportedChainIdStr(e.to_string()))
        } else {
            Err(crate::Error::UnsupportedChainIdStr(s.to_string()))
        }
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
        ChainId::try_from_primitive(v)
            .map_err(|e| D::Error::invalid_value(Unexpected::Str(&e.to_string()), &"valid ChainId"))
    }
}
