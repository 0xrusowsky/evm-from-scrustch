use ethereum_types::{U256, U64};
use serde::Deserialize;

use crate::types::{
    hex_string_to_address,
    Address,
    Bytes,
};

// EVM environment configuration.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Hash)]
pub struct Env {
    // Chain ID
    #[serde(default, rename = "chainId")]
    pub chain_id: U64,
    // Configuration of the block the transaction is in.
    pub block: BlockEnv,
    // Configuration of the transaction that is being executed.
    pub tx: TxEnv,
}

// The block environment.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BlockEnv {
    // The number of ancestor blocks of this block (block height).
    pub number: U256,
    // Proposer or miner or address that created and signed the block.
    // Beneficiary address of all the gas spent in the block.
    pub coinbase: Address,
    // The timestamp of the block in seconds since the UNIX epoch.
    pub timestamp: U256,
    // The gas limit of the block.
    pub gas_limit: U256,
    // The base fee per gas after London [EIP-1559].
    #[serde(rename = "baseFee")]
    pub base_fee: U256,
    // The difficulty of the block.
    // Unused after the merge and replaced by `prev_randao`.
    pub difficulty: Option<U256>,
    // The output of the randomness beacon provided by the beacon chain.
    // Replaces `difficulty` after the merge [EIP-4399].
    #[serde(rename = "prevRandao")]
    pub prev_randao: Option<U256>,
}

impl BlockEnv {
    // Clears environment and resets fields to default values.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Default for BlockEnv {
    fn default() -> Self {
        Self {
            number: U256::zero(),
            coinbase: Address::zero(),
            timestamp: U256::from(1),
            gas_limit: U256::MAX,
            base_fee: U256::zero(),
            difficulty: None,
            prev_randao: Some(U256::zero()),
        }
    }
}

// The transaction environment.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TxEnv {
    // EOA that originated the transaction (signer).
    #[serde(rename = "origin", deserialize_with = "hex_string_to_address")]
    pub originator: Address,
    // The destination of the transaction.
    #[serde(rename = "to", deserialize_with = "hex_string_to_address")]
    pub recipient: Address,
    // The value sent to `transact_to`.
    pub value: U256,
    // The data of the transaction.
    pub data: Bytes,
    // The nonce of the transaction. If set to `None`, no checks are performed.
    pub nonce: Option<u64>,
    // The gas limit of the transaction.
    #[serde(rename = "gaslimit")]
    pub gas_limit: u64,
    // The gas price of the transaction.
    #[serde(rename = "gasprice")]
    pub gas_price: U256,
    // The priority fee per gas after London [EIP-1559].
    pub gas_priority_fee: Option<U256>,
    // A list of addresses and storage keys that the transaction plans to access.
    pub access_list: Vec<(Address, Vec<U256>)>,
}

impl TxEnv {
    // Clears environment and resets fields to default values.
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Default for TxEnv {
    fn default() -> Self {
        Self {
            originator: Address::zero(),
            recipient: Address::zero(),
            value: U256::zero(),
            data: Bytes::new(),
            nonce: None,
            gas_limit: u64::MAX,
            gas_price: U256::zero(),
            gas_priority_fee: None,
            access_list: Vec::new(),
        }
    }
}