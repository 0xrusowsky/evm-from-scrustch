use crate::primitives::{Address, Bytes, Env, U256};
use crate::Call;

// EVM contract information.
#[derive(Clone, Debug, Default)]
pub struct Contract {
    // Contract address
    pub address: Address,
    // Bytecode of the contract.
    pub code: Bytes,
    // Caller of the contract.
    pub caller: Address,
    // Contract calldata
    pub calldata: Bytes,
    // Value sent to the contract.
    pub value: U256,
}

impl Contract {
    // Instantiates a new contract by analyzing the given bytecode.
    #[inline]
    pub fn new(
        address: Address,
        bytecode: Bytes,
        caller: Address,
        calldata: Bytes,
        value: U256,
    ) -> Self {
        Self {
            address,
            code: bytecode,
            caller,
            calldata,
            value,
        }
    }

    // Creates a new contract from the given [`Env`].
    pub fn new_env(env: &Env, bytecode: Bytes) -> Self {
        Self::new(
            env.tx.code_target.clone(),
            bytecode,
            env.tx.sender.clone(),
            env.tx.data.clone(),
            env.tx.value,
        )
    }

    // Creates a new contract from a Call.
    pub fn new_call(call: &Call, bytecode: Bytes) -> Self {
        Self::new(
            call.recipient,
            bytecode,
            call.sender,
            call.data,
            call.value,
        )
    }

    pub fn data_size(&self) -> usize {
        (&self.calldata.len() + 31) / 32 * 32
    }

    pub fn is_zero(&self) -> bool {
        self.value == U256::zero()
    }
}