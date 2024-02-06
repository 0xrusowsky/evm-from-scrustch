use crate::types::{Address, Bytes, Bytes32, U256};
use crate::call::{Call, CallResult};
use crate::logs::Log;
use crate::env::Env;
/// EVM context host.
pub trait Host {
    /// Returns a mutable reference to the environment.
    fn env(&mut self) -> &mut Env;

    /// Load an account.
    fn load_account(&mut self, address: &Address) -> Option<(bool, bool)>;

    /// Get the block hash of the given block `number`.
    fn block_hash(&mut self, number: U256) -> Bytes32;

    /// Get balance of `address` and if the account is cold.
    fn balance(&mut self, address: &Address) -> U256;

    /// Get balance of `address` and if the account is cold.
    fn nonce(&mut self, address: &Address) -> U256;

    /// Get code of `address` and if the account is cold.
    fn code(&mut self, address: &Address) -> &Bytes;

    /// Get code of `address` and if the account is cold.
    fn code_hash(&mut self, address: &Address) -> Bytes32;

    /// Get code of `address` and if the account is cold.
    fn code_size(&mut self, address: &Address) -> usize;

    /// Get storage value of `address` at `index` and if the account is cold.
    fn sload(&mut self, address: &Address, index: U256) -> Bytes32;

    /// Set storage value of account address at index.
    fn sstore(
        &mut self,
        address: &Address,
        index: U256,
        value: Bytes32,
    );

    /// Emit a log owned by `address` with given `Log`.
    fn log(&mut self, log: Log);

    /// Mark `address` to be deleted, with funds transferred to `target`.
    fn selfdestruct(&mut self, from: &Address, to: &Address, value: U256) -> Result<(), String>;
    fn execute_call(&mut self, call: Call) -> CallResult;
    fn create_call(&mut self, address: Address, value: U256, code: Bytes) -> CallResult;
}