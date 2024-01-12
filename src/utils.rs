use ethereum_types::{U64, H160, U256};
use serde::Deserialize;
use serde::de::{self, Deserializer};

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Address(H160);

impl Address {
    pub fn from_slice(slice: &[u8]) -> Self {
        let len = slice.len();
        let mut address = [0u8; 20];
        
        if len >= 20 {
            address.copy_from_slice(&slice[..20]);
        } else {
            address[20-len..].copy_from_slice(slice);
        }
        Address(H160::from(address))
    }

    pub fn to_u256(&self) -> U256 {
        let mut bytes = [0u8; 32];
        bytes[12..32].copy_from_slice(self.0.as_bytes());
        U256::from_big_endian(&bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// Convert U64 to U256
pub fn u64_to_u256(number: U64) -> U256 {
    let mut bytes = [0u8; 32];
    bytes[24..32].copy_from_slice(&number.0[0].to_be_bytes());
    U256::from_big_endian(&bytes)
}

// Custom deserializers to convert hex strings from EVM Test

pub fn hex_string_to_u64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Convert hex string to bytes
    hex::decode(trimmed).map_err(de::Error::custom)
}

pub fn hex_string_to_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Convert hex string to bytes
    hex::decode(trimmed).map_err(de::Error::custom)
}

pub fn hex_string_to_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Prepend "0" if the length of trimmed is odd
    let padded = if trimmed.len() % 2 != 0 {
        format!("0{}", trimmed)
    } else {
        trimmed.to_string()
    };
    // Convert hex string to bytes
    let bytes = hex::decode(&padded).map_err(de::Error::custom)?;
    Ok(Address::from_slice(&bytes))
}

pub fn hex_string_to_address_option<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            // Remove "0x" prefix if present
            let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
            // Prepend "0" if the length of trimmed is odd
            let padded = if trimmed.len() % 2 != 0 {
                format!("0{}", trimmed)
            } else {
                trimmed.to_string()
            };
            // Convert hex string to bytes
            let bytes = hex::decode(&padded).map_err(de::Error::custom)?;
            Ok(Some(Address::from_slice(&bytes)))
        }
        None => Ok(None), // If the string is None, return None
    }
}
