use ethereum_types::{U64, U256, H256};
use serde::Deserialize;

use crate::types::{
    Address,
    // hex_string_to_u64,
    hex_string_to_address_option,
};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Block {
    /// Chain ID
    #[serde(default, rename = "chainId")]
    pub chain_id: U64,
    /// Hash of the block
    pub hash: Option<H256>,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Hash of the parent
    #[serde(default)]
    pub parent_hash: H256,
    /// Hash of the uncles
    #[serde(default)]
    pub uncles_hash: H256,
    /// Miner/author's address. None if pending.
    #[serde(default, rename = "miner")]
    pub author: Option<Address>,
    /// State root hash
    #[serde(default, rename = "stateRoot")]
    pub state_root: H256,
    /// Transactions root hash
    #[serde(default, rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// Transactions receipts root hash
    #[serde(default, rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Gas Used
    #[serde(default, rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[serde(default, rename = "gasLimit")]
    pub gas_limit: U256,
    /// Logs bloom
    // #[serde(rename = "logsBloom")]
    // pub logs_bloom: Option<Bloom>,
    /// Timestamp
    #[serde(default)]
    pub timestamp: U256,
    /// Previous RANDAO
    #[serde(default, rename = "prevRandao")]
    pub prev_randao: Option<U256>,
    /// Difficulty
    #[serde(default)]
    pub difficulty: Option<U256>,
    /// Total difficulty
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
    /// Transactions
    // #[serde(bound = "TX: Serialize + serde::de::DeserializeOwned", default)]
    // pub transactions: Vec<TX>,
    /// Size in bytes
    pub size: Option<U256>,
    /// Nonce
    pub nonce: Option<U256>,
    /// Base fee per unit of gas (if past London)
    #[serde(rename = "baseFee")]
    pub base_fee: Option<U256>,
    /// Beneficiary address (if past London)
    #[serde(default, rename = "coinbase", deserialize_with = "hex_string_to_address_option")]
    pub beneficiary: Option<Address>,
    /// Blob gas used (if past Cancun)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blobGasUsed")]
    pub blob_gas_used: Option<U256>,
    /// Excess blob gas (if past Cancun)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excessBlobGas")]
    pub excess_blob_gas: Option<U256>,
    /// Withdrawals root hash (if past Shanghai)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withdrawalsRoot")]
    pub withdrawals_root: Option<H256>,
    /// Withdrawals (if past Shanghai)
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // pub withdrawals: Option<Vec<Withdrawal>>,
    /// Parent beacon block root (if past Cancun)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentBeaconBlockRoot")]
    pub parent_beacon_block_root: Option<H256>,
}

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn number(&self) -> Option<U64> {
        self.number
    }

    pub fn hash(&self) -> Option<H256> {
        self.hash
    }

    pub fn parent_hash(&self) -> H256 {
        self.parent_hash
    }

    pub fn author(&self) -> Option<Address> {
        self.author
    }

    pub fn state_root(&self) -> H256 {
        self.state_root
    }

    pub fn transactions_root(&self) -> H256 {
        self.transactions_root
    }

    pub fn receipts_root(&self) -> H256 {
        self.receipts_root
    }

    pub fn gas_used(&self) -> U256 {
        self.gas_used
    }

    pub fn gas_limit(&self) -> U256 {
        self.gas_limit
    }

    pub fn timestamp(&self) -> U256 {
        self.timestamp
    }

    pub fn prev_randao(&self) -> Option<U256> {
        self.prev_randao
    }

    pub fn difficulty(&self) -> Option<U256> {
        self.difficulty
    }

    pub fn total_difficulty(&self) -> Option<U256> {
        self.total_difficulty
    }

    pub fn size(&self) -> Option<U256> {
        self.size
    }

    pub fn nonce(&self) -> Option<U256> {
        self.nonce
    }

    pub fn base_fee(&self) -> Option<U256> {
        self.base_fee
    }

    pub fn beneficiary(&self) -> Option<Address> {
        self.beneficiary
    }

    pub fn blob_gas_used(&self) -> Option<U256> {
        self.blob_gas_used
    }

    pub fn excess_blob_gas(&self) -> Option<U256> {
        self.excess_blob_gas
    }

    pub fn chain_id(&self) -> U64 {
        self.chain_id
    }
}