use std::fmt;
use std::ops::{Index, IndexMut, Range};
use serde::Deserialize;
use serde::de::{self, Deserializer};
use ethereum_types::{U64, H160, U256, U512};

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new() -> Bytes {
        Bytes(Vec::new())
    }

    pub fn from_vec(vec: Vec<u8>) -> Bytes {
        Bytes(vec)
    }

    pub fn from_slice(slice: &[u8]) -> Bytes {
        Bytes(slice.to_vec())
    }

    pub fn from_byte(byte: u8) -> Bytes {
        Bytes(vec![byte])
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn as_usize(&self) -> usize {
        // Take the least significant part that fits into usize
        self.as_bytes32().as_usize()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn resize(&mut self, new_size: usize, value: u8) {
        self.0.resize(new_size, value);
    }
    
    // Conversion from/to Bytes32
    pub fn as_bytes32(&self) -> Bytes32 {
        let vec = self.0.to_vec();
        let len = vec.len();
        let mut bytes = [0u8; 32];
        if len < 32 {
            bytes[..len].copy_from_slice(&vec);
        } else {
            bytes.copy_from_slice(&vec[len-32..len]);
        }
        Bytes32::from_vec(bytes.to_vec())
    }

    pub fn from_bytes32(bytes: Bytes32) -> Bytes {
        bytes.as_bytes()
    }
}

// Immutable indexing
impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

// Mutable indexing
impl IndexMut<usize> for Bytes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IndexMut<Range<usize>> for Bytes {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl fmt::UpperHex for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Bytes32(Vec<u8>);

impl Bytes32 {
    pub fn new() -> Bytes32 {
        Bytes32::from_vec(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_usize(&self) -> usize {
        self.to_u256().as_usize()
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        if index < 32 {self.0[index]} else { 0 }
    }

    // Conversion from/to Vec<u8>
    pub fn from_vec(vec: Vec<u8>) -> Bytes32 {
        let len = vec.len();
        let mut bytes = [0u8; 32];
        if len < 32 {
            bytes[32-len..32].copy_from_slice(&vec);
        } else {
            bytes.copy_from_slice(&vec[len-32..len]);
        }
        Bytes32(bytes.to_vec())
    }

    pub fn from_slice(slice: &[u8]) -> Bytes32 {
        Bytes32::from_vec(slice.to_vec())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    // Conversion from/to Bytes
    pub fn as_bytes(&self) -> Bytes {
        Bytes(self.0.clone())
    }

    pub fn from_bytes(bytes: Bytes) -> Bytes32 {
        bytes.as_bytes32()
    }

    // Conversion from/to U512
    pub fn to_u512(&self) -> U512 {
        let len = self.0.len();
        let mut bytes = [0u8; 64];
        bytes[64-len..64].copy_from_slice(&self.0);
        U512::from_big_endian(&bytes)
    }

    pub fn from_u512(number: U512) -> Bytes32 {
        let mut bytes = [0u8; 32];
        number.to_big_endian(&mut bytes);
        Bytes32::from_vec(bytes.to_vec())
    }

    // Conversion from/to U256
    pub fn to_u256(&self) -> U256 {
        let len = self.0.len();
        let mut bytes = [0u8; 32];
        if len < 32 {
            bytes[32-len..32].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len-32..len]);
        }
        U256::from_big_endian(&bytes)
    }

    pub fn from_u256(number: U256) -> Bytes32 {
        let mut bytes = [0u8; 32];
        number.to_big_endian(&mut bytes);
        // println!("Bytes32::from_u256({:#?}) -> {:#?}", number, bytes);
        Bytes32::from_vec(bytes.to_vec())
    }

    // Conversion from/to U64
    pub fn to_u64(&self) -> U64 {
        let len = self.0.len();
        let mut bytes = [0u8; 8];
        if len < 8 {
            bytes[8-len..8].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len-8..len]);
        }
        U64::from_big_endian(&bytes)
    }

    pub fn from_u64(number: U64) -> Bytes32 {
        let mut bytes = [0u8; 8];
        number.to_big_endian(&mut bytes);
        Bytes32::from_vec(bytes.to_vec())
    }

    // Conversion from/to H160
    pub fn to_h160(&self) -> H160 {
        let len = self.0.len();
        let mut bytes = [0u8; 20];
        if len < 20 {
            bytes[20-len..20].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len-20..len]);
        }
        H160::from(&bytes)
    }

    pub fn from_h160(address: H160) -> Bytes32 {
        let mut bytes = [0u8; 32];
        bytes[12..32].copy_from_slice(address.as_bytes());
        Bytes32::from_vec(bytes.to_vec())
    }

    // Conversion from/to Address
    pub fn to_address(&self) -> Address {
        Address(self.to_h160())
    }

    pub fn from_address(address: Address) -> Bytes32 {
        Self::from_h160(address.0)
    }
}

// Immutable indexing
impl Index<usize> for Bytes32 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for Bytes32 {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

// Mutable indexing
impl IndexMut<usize> for Bytes32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IndexMut<Range<usize>> for Bytes32 {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl fmt::UpperHex for Bytes32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub struct Address(H160);

impl Address {
    pub fn from_slice(slice: &[u8]) -> Self {
        Bytes32::from_vec(slice.to_vec()).to_address()
    }

    pub fn from_u256(number: U256) -> Self {
       Bytes32::from_u256(number).to_address()
    }

    pub fn to_u256(&self) -> U256 {
        Bytes32::from_address(*self).to_u256()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_bytes32(&self) -> Bytes32 {
        Bytes32::from_address(*self)
    }
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

pub fn hex_string_to_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Convert hex string to bytes
    let bytes = hex::decode(trimmed).map_err(de::Error::custom)?;
    Ok(Bytes::from_vec(bytes))
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
