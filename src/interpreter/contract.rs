use ethereum_types::U256;
use crate::call::CallContext;
use crate::primitives::{types::*, env::Env};

// EVM contract information.
#[derive(Clone, Debug, Default)]
pub struct Contract {
    // Contract input data.
    pub calldata: Bytes,
    // Contract bytecode.
    pub code: Bytes,
    // Bytecode hash.
    pub hash: Bytes32,
    // Contract address
    pub address: Address,
    // Caller of the EVM.
    pub caller: Address,
    // Value send to contract.
    pub value: U256,
}

impl Contract {
    // Creates a new contract from the given `Env`.
    pub fn from_env(env: &Env, bytecode: Bytes, hash: Bytes32) -> Self {
        Self::new(
            env.tx.data.clone(),
            bytecode,
            hash,
            env.tx.recipient,
            env.tx.caller,
            env.tx.value,
        )
    }

    // Creates a new contract from the given `CallContext`.
    pub fn from_call(
        calldata: Bytes,
        bytecode: Bytes,
        hash: Bytes32,
        ctx: &CallContext,
    ) -> Self {
        Self::new(
            calldata,
            bytecode,
            hash,
            ctx.address,
            ctx.caller,
            ctx.value,
        )
    }

    pub fn calldata_size(&self) -> usize {
        (&self.calldata.len() + 31) / 32 * 32
    }
}