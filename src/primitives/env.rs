use serde::Deserialize;

use crate::types::{
    Bytes,
    Address,
    U64, U256,
    hex_string_to_bytes,
    hex_string_to_address, 
    hex_string_to_address_option
};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Env {
    /// Call
    #[serde(default)]
    pub call: Call,
    /// Block
    #[serde(default)]
    pub block: Block,
}

impl Env {
    pub fn new(call: Call, block: Block) -> Self {
        Self { call, block }
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Block {
    /// Chain ID
    #[serde(default, rename = "chainId")]
    pub chain_id: U64,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Miner/author's address. None if pending.
    #[serde(default, rename = "miner")]
    pub author: Option<Address>,
    /// Gas Used
    #[serde(default, rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[serde(default, rename = "gasLimit")]
    pub gas_limit: U256,
    /// Timestamp
    #[serde(default)]
    pub timestamp: U256,
    /// Previous RANDAO
    #[serde(default, rename = "prevRandao")]
    pub prev_randao: Option<U256>,
    /// Difficulty
    #[serde(default)]
    pub difficulty: Option<U256>,
    /// Base fee per unit of gas (if past London)
    #[serde(rename = "baseFee")]
    pub base_fee: Option<U256>,
    /// Beneficiary address (if past London)
    #[serde(
        default,
        rename = "coinbase",
        deserialize_with = "hex_string_to_address_option"
    )]
    pub beneficiary: Option<Address>,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Call {
    #[serde(default, rename = "from", deserialize_with = "hex_string_to_address")]
    pub sender: Address,
    #[serde(default, rename = "to", deserialize_with = "hex_string_to_address")]
    pub recipient: Address,
    #[serde(default, rename = "origin", deserialize_with = "hex_string_to_address")]
    pub originator: Address,
    #[serde(default, rename = "gasprice")]
    pub gas_price: U256,
    #[serde(default)]
    pub available_gas: U256,
    #[serde(default)]
    pub code_target: Address,
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    pub data: Bytes,
    #[serde(default)]
    pub value: U256,
    #[serde(default)]
    view: bool,
    #[serde(default, deserialize_with = "hex_string_to_bytes")]
    result: Bytes,
}

impl Call {
    pub fn new(
        sender: Address,
        recipient: Address,
        originator: Address,
        gas_price: U256,
        available_gas: U256,
        code_target: Address,
        data: Bytes,
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
            result: Bytes::new(),
        }
    }

    // Getters

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }

    pub fn data_size(&self) -> usize {
        (&self.data.len() + 31) / 32 * 32
    }

    pub fn is_static(&self) -> bool {
        self.view
    }

    pub fn is_zero(&self) -> bool {
        self.value == U256::zero()
    }

    pub fn result(&self) -> Bytes {
        self.result.clone()
    }

    // Setters
    pub fn set_result(&mut self, result: Bytes) {
        self.result = result;
    }
}
