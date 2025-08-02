use std::{fmt::Display, str::FromStr};

use alloy::{
    primitives::Address,
    signers::k256::sha2::{Digest, Sha256},
};
use serde::{Deserialize, Serialize};

use crate::chain_id::ChainId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MultichainAddress {
    Ethereum {
        raw: Address,
        chain_id: Option<ChainId>,
    },
    Tron {
        raw: Address,
    },
}

impl MultichainAddress {
    pub fn without_chain_id(self) -> Self {
        match self {
            MultichainAddress::Ethereum { raw, .. } => MultichainAddress::Ethereum {
                raw,
                chain_id: None,
            },
            _ => self,
        }
    }

    pub fn as_raw(&self) -> Address {
        match self {
            MultichainAddress::Ethereum { raw, .. } => *raw,
            MultichainAddress::Tron { raw } => *raw,
        }
    }

    pub fn get_chain_id(&self) -> Option<ChainId> {
        match self {
            MultichainAddress::Ethereum { chain_id, .. } => *chain_id,
            MultichainAddress::Tron { .. } => Some(ChainId::Tron),
        }
    }
}

impl Default for MultichainAddress {
    fn default() -> Self {
        MultichainAddress::Ethereum {
            raw: Address::default(),
            chain_id: None,
        }
    }
}

impl Display for MultichainAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MultichainAddress::Ethereum { raw, chain_id } => {
                if let Some(chain_id) = chain_id {
                    format!(
                        "{}@{}",
                        raw.to_checksum_buffer(Some(*chain_id as u64)),
                        chain_id.to_network_name()
                    )
                } else {
                    raw.to_checksum(None)
                }
            }
            MultichainAddress::Tron { raw } => {
                let mut bytes = raw.to_vec();
                bytes.insert(0, 0x41);
                let checksum = Sha256::digest(Sha256::digest(&bytes));
                bytes.extend(&checksum[..4]);
                bs58::encode(bytes).into_string()
            }
        };
        write!(f, "{str}")
    }
}

impl Serialize for MultichainAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MultichainAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        MultichainAddress::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for MultichainAddress {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(eth_address) = Address::from_str(value) {
            return Ok(MultichainAddress::Ethereum {
                raw: eth_address,
                chain_id: None,
            });
        } else if let Ok(bytes) = bs58::decode(&value).into_vec() {
            if bytes.len() == 1 + 20 + 4 && bytes[0] == 0x41 {
                return Ok(MultichainAddress::Tron {
                    raw: Address::from_slice(&bytes[1..21]),
                });
            }
        } else if let Some(idx) = value.find("@") {
            if let Some((left, right)) = value.split_at_checked(idx) {
                let right = &right[1..]; // Skip '@'
                let chain_id = ChainId::from_str(right)?;
                if let Ok(address) = Address::from_str(left) {
                    return Ok(match chain_id {
                        ChainId::Tron => MultichainAddress::Tron { raw: address },
                        _ => MultichainAddress::Ethereum {
                            raw: address,
                            chain_id: Some(chain_id),
                        },
                    });
                }
            }
        }

        Err(crate::Error::MultichainAddressDecodeFailed(
            value.to_string(),
        ))
    }
}

impl From<Address> for MultichainAddress {
    fn from(address: Address) -> Self {
        MultichainAddress::Ethereum {
            raw: address,
            chain_id: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use alloy::primitives::Address;

    use crate::multichain_address::MultichainAddress;

    #[test]
    fn test_tron() {
        assert_eq!(
            MultichainAddress::Tron {
                raw: "0x5bc44f18b91f55540d11d612c08e4faad619eb55"
                    .parse()
                    .unwrap(),
            }
            .to_string(),
            "TJLRfJUAHPRxoizJeyYFFZ7nEHit4L9FfE"
        );
    }

    #[test]
    fn test_try_from_eth() {
        let result =
            MultichainAddress::from_str("0x5bc44f18b91f55540d11d612c08e4faad619eb55").unwrap();

        assert_eq!(
            result.to_string(),
            // checksum
            Address::from_str("0x5bc44f18b91f55540d11d612c08e4faad619eb55")
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test_try_from_tron() {
        let result = MultichainAddress::from_str("TJLRfJUAHPRxoizJeyYFFZ7nEHit4L9FfE").unwrap();

        assert_eq!(result.to_string(), "TJLRfJUAHPRxoizJeyYFFZ7nEHit4L9FfE");
    }
}
