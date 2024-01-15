use ethereum_types::{H160, U256, U512, U64};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{Index, IndexMut, Range};

// --- TYPE: BYTES -----------------------------------------------------------
//  A wrapper around Vec<u8> that represents an arbitrary number of bytes.
//  Implements bitwise operations and conversion from/to other types that are
//  used in EVM.

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
        // Take the least significant bytes that fit into usize
        self.to_u512().as_usize()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn resize(&mut self, new_size: usize, value: u8) {
        self.0.resize(new_size, value);
    }

    pub fn zero() -> Bytes {
        Bytes::from_byte(0)
    }

    pub fn one() -> Bytes {
        Bytes::from_byte(1)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    // Conversion from/to Bytes32
    pub fn as_bytes32(&self) -> Bytes32 {
        let vec = self.0.to_vec();
        let len = vec.len();
        let mut bytes = [0u8; 32];
        if len < 32 {
            bytes[..len].copy_from_slice(&vec);
        } else {
            bytes.copy_from_slice(&vec[len - 32..len]);
        }
        Bytes32::from_vec(bytes.to_vec())
    }

    pub fn from_bytes32(bytes: Bytes32) -> Bytes {
        bytes.as_bytes()
    }

    // Conversion from/to U512
    pub fn to_u512(&self) -> U512 {
        let len = self.0.len();
        let mut bytes = [0u8; 64];
        bytes[64 - len..64].copy_from_slice(&self.0);
        U512::from_big_endian(&bytes)
    }

    pub fn from_u512(number: U512) -> Bytes32 {
        let mut bytes = [0u8; 32];
        number.to_big_endian(&mut bytes);
        Bytes32::from_vec(bytes.to_vec())
    }
}

// -- TYPE: BYTES32 -----------------------------------------------------------
//  A wrapper around Vec<u8> that represents an 32 bytes.
//  Implements bitwise operations and conversion from/to other types that are
//  used in EVM.

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Bytes32(Vec<u8>);

impl Bytes32 {
    pub fn new() -> Bytes32 {
        Bytes32::from_vec(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn as_usize(&self) -> usize {
        self.to_u256().as_usize()
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        if index < 32 {
            self.0[index]
        } else {
            0
        }
    }

    pub fn from_vec(vec: Vec<u8>) -> Bytes32 {
        let len = vec.len();
        let mut bytes = [0u8; 32];
        if len < 32 {
            bytes[32 - len..32].copy_from_slice(&vec);
        } else {
            bytes.copy_from_slice(&vec[len - 32..len]);
        }
        Bytes32(bytes.to_vec())
    }

    pub fn from_slice(slice: &[u8]) -> Bytes32 {
        Bytes32::from_vec(slice.to_vec())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn zero() -> Bytes32 {
        let bytes = [0u8; 32];
        Bytes32::from_slice(&bytes)
    }

    pub fn one() -> Bytes32 {
        let mut bytes = [0u8; 32];
        bytes[31] = 1;
        Bytes32::from_slice(&bytes)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
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
        bytes[64 - len..64].copy_from_slice(&self.0);
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
            bytes[32 - len..32].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len - 32..len]);
        }
        U256::from_big_endian(&bytes)
    }

    pub fn from_u256(number: U256) -> Bytes32 {
        let mut bytes = [0u8; 32];
        number.to_big_endian(&mut bytes);
        Bytes32::from_vec(bytes.to_vec())
    }

    // Conversion from/to U64
    pub fn to_u64(&self) -> U64 {
        let len = self.0.len();
        let mut bytes = [0u8; 8];
        if len < 8 {
            bytes[8 - len..8].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len - 8..len]);
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
            bytes[20 - len..20].copy_from_slice(&self.0);
        } else {
            bytes.copy_from_slice(&self.0[len - 20..len]);
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

// -- COMMON TRAITS -----------------------------------------------------------

// Immutable indexing
impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl Index<usize> for Bytes32 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Immutable indexing of a range
impl Index<Range<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
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
impl IndexMut<usize> for Bytes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl IndexMut<usize> for Bytes32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// Mutable indexing of a range
impl IndexMut<Range<usize>> for Bytes {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl IndexMut<Range<usize>> for Bytes32 {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// Formatting as hex
impl fmt::UpperHex for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
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
impl fmt::UpperHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 .0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

// Bitwise operations (generic implementation)
fn bitnot(a: Vec<u8>) -> Vec<u8> {
    a.iter().map(|&x| !x).collect()
}

fn bitwise_operation(a: Vec<u8>, b: Vec<u8>, operator: fn(x: u8, y: u8) -> u8) -> Vec<u8> {
    let len = std::cmp::max(a.len(), b.len());
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        result.push(operator(x, y));
    }
    result
}

fn bitand(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    bitwise_operation(a, b, |x, y| x & y)
}

fn bitor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    bitwise_operation(a, b, |x, y| x | y)
}

fn bitxor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    bitwise_operation(a, b, |x, y| x ^ y)
}

// Bitwise operations (trait implementations)
impl BitAnd for Bytes {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bytes(bitand(self.0, rhs.0))
    }
}

impl BitAnd for Bytes32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bytes32(bitand(self.0, rhs.0))
    }
}

impl BitOr for Bytes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bytes(bitor(self.0, rhs.0))
    }
}

impl BitOr for Bytes32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bytes32(bitor(self.0, rhs.0))
    }
}

impl BitXor for Bytes {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bytes(bitxor(self.0, rhs.0))
    }
}

impl BitXor for Bytes32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bytes32(bitxor(self.0, rhs.0))
    }
}

impl Not for Bytes {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bytes(bitnot(self.0))
    }
}

impl Not for Bytes32 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bytes32(bitnot(self.0))
    }
}

// -- UTILS -------------------------------------------------------------------

// Custom deserializers to convert hex strings from EVM Test

pub fn hex_string_to_u64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed = match s.strip_prefix("0x") {
        Some(stripped) => stripped,
        None => &s,
    };
    hex::decode(trimmed).map_err(de::Error::custom)
}

pub fn hex_string_to_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed = match s.strip_prefix("0x") {
        Some(stripped) => stripped,
        None => &s,
    };
    let bytes = hex::decode(trimmed).map_err(de::Error::custom)?;
    Ok(Bytes::from_vec(bytes))
}

pub fn hex_string_to_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed = match s.strip_prefix("0x") {
        Some(stripped) => stripped,
        None => &s,
    };
    let padded = if trimmed.len() % 2 != 0 {
        format!("0{}", trimmed)
    } else {
        trimmed.to_string()
    };
    let bytes = hex::decode(padded).map_err(de::Error::custom)?;
    Ok(Address::from_slice(&bytes))
}

pub fn hex_string_to_address_option<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let trimmed = match s.strip_prefix("0x") {
                Some(stripped) => stripped,
                None => &s,
            };
            let padded = if trimmed.len() % 2 != 0 {
                format!("0{}", trimmed)
            } else {
                trimmed.to_string()
            };
            let bytes = hex::decode(padded).map_err(de::Error::custom)?;
            Ok(Some(Address::from_slice(&bytes)))
        }
        None => Ok(None),
    }
}
