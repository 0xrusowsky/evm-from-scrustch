use serde::Deserialize;
use serde::de::{self, Deserializer};
use ethereum_types::U256;

use crate::utils::Address;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Call {
    #[serde(default, rename = "from", deserialize_with = "hex_string_to_address")]
    sender: Address,
    #[serde(default, rename = "to", deserialize_with = "hex_string_to_address")]
    recipient: Address,
    #[serde(default, rename = "origin", deserialize_with = "hex_string_to_address")]
    originator: Address,
    #[serde(default, rename = "gasprice")]
    gas_price: U256,
    #[serde(default)]
    available_gas: U256,
    #[serde(default)]
    code_target: Address,
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    data: Vec<u8>,
    #[serde(default)]
    value: U256,
    #[serde(default)]
    view: bool,
}

impl Call {
    pub fn new(
        sender: Address,
        recipient: Address,
        originator: Address,
        gas_price: U256,
        available_gas: U256,
        code_target: Address,
        data: Vec<u8>,
        value: U256,
        view: bool,
    ) -> Self {
        Self {
            sender,
            recipient,
            originator,
            code_target,
            available_gas,
            gas_price,
            data,
            value,
            view,
        }
    }

    // Getters
    pub fn sender(&self) -> Address {
        self.sender
    }

    pub fn recipient(&self) -> Address {
        self.recipient
    }

    pub fn originator(&self) -> Address {
        self.originator
    }

    pub fn gas_price(&self) -> U256 {
        self.gas_price
    }

    pub fn available_gas(&self) -> U256 {
        self.available_gas
    }

    pub fn code_target(&self) -> Address {
        self.code_target
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_size(&self) -> usize {
        (&self.data.len() + 31) / 32 * 32
    }

    pub fn value(&self) -> U256 {
        self.value
    }

    pub fn view(&self) -> bool {
        self.view
    }
}


// Custom deserializers to convert hex strings from EVM Test

fn hex_string_to_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Convert hex string to bytes
    hex::decode(trimmed).map_err(de::Error::custom)
}

fn hex_string_to_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove "0x" prefix if present
    let trimmed = if s.starts_with("0x") { &s[2..] } else { &s };
    // Convert hex string to bytes
    let bytes = hex::decode(trimmed).map_err(de::Error::custom)?;
    
    Ok(Address::from_slice(&bytes))
}