use ethereum_types::U256;
use serde::Deserialize;

use crate::types::{hex_string_to_address, hex_string_to_bytes, Address, Bytes};

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
    data: Bytes,
    #[serde(default)]
    value: U256,
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

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }

    pub fn data_size(&self) -> usize {
        (&self.data.len() + 31) / 32 * 32
    }

    pub fn value(&self) -> U256 {
        self.value
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
