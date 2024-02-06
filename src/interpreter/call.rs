use crate::primitives::{Address, Bytes, Bytes32, TxEnv, U256};

/// Inputs for a call.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    // Call sender (in solidity `msg.from`)
    pub sender: Address,
    // Call receiver
    pub recipient: Address,
    // Transaction originator (in solidity `tx.origin`)
    pub originator: Address,
    // Contract address of the code to be executed
    pub code_target: Address,
    // Call data
    pub data: Bytes,
    // Value transferred in the call
    pub value: U256,
    // Whether it is a view only call or not
    pub view: bool,
    /// Call type.
    pub scheme: CallScheme,
}

/// Call schemes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallScheme {
    CALL,
    CALLCODE,
    DELEGATECALL,
    STATICCALL,
}

/// Result of a call.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallResult {
    // Whether the transaction was successful (1) or not (0)
    pub success: Bytes32,
    // Result of the transaction execution
    pub result: Bytes,
}

impl Call {
    /// Creates new call inputs based on the transaction environment.
    pub fn new_env(tx_env: &TxEnv) -> Self {
        Call {
            sender: tx_env.sender.clone(),
            recipient: tx_env.recipient.clone(),
            originator: tx_env.originator.clone(),
            code_target: tx_env.code_target.clone(),
            data: tx_env.data.clone(),
            value: tx_env.value,
            view: false,
            scheme: CallScheme::CALL,
        }
    }
}