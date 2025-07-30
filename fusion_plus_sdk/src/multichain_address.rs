use std::{fmt::Display, str::FromStr};

use alloy::{
    primitives::Address,
    signers::k256::sha2::{Digest, Sha256},
};

use crate::chain_id::ChainId;

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
}

impl Display for MultichainAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MultichainAddress::Ethereum { raw, chain_id } => {
                if let Some(chain_id) = chain_id {
                    format!("{}:{:?}", chain_id.to_network_name(), raw)
                } else {
                    raw.to_string()
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

impl TryFrom<String> for MultichainAddress {
    type Error = crate::Error;

    fn try_from(value: String) -> crate::Result<Self> {
        if let Ok(eth_address) = Address::from_str(&value) {
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
        } else if let Some(idx) = value.find(":") {
            if let Some((left, right)) = value.split_at_checked(idx) {
                if let Ok(chain_id) = ChainId::from_network_name(left) {
                    if let Ok(address) = Address::from_str(right) {
                        return Ok(MultichainAddress::Ethereum {
                            raw: address,
                            chain_id: Some(chain_id),
                        });
                    }
                }
            }
        }

        Err(crate::Error::MultichainAddressDecodeFailed(value))
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
            MultichainAddress::try_from("0x5bc44f18b91f55540d11d612c08e4faad619eb55".to_string())
                .unwrap();

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
        let result =
            MultichainAddress::try_from("TJLRfJUAHPRxoizJeyYFFZ7nEHit4L9FfE".to_string()).unwrap();

        assert_eq!(result.to_string(), "TJLRfJUAHPRxoizJeyYFFZ7nEHit4L9FfE");
    }
}
