use ethereum_types::{H160, U256};
use serde::Deserialize;

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