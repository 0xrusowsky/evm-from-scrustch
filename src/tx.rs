// use ethereum_types::{U256, H160};

// pub enum Transaction {
//     Legacy(LegacyTx),
//     Eip2930(Eip2930Tx),
//     Eip1559(Eip1559Tx),
// }

// impl Transaction {
//     fn tx_type(&self) -> u8 {
//         match self {
//             Transaction::Legacy(_) => 0,
//             Transaction::Eip2930(_) => 1,
//             Transaction::Eip1559(_) => 2,
//         }
//     }
// }

// struct LegacyTx {
//     nonce: U256,
//     chain_id: U256,
//     from: H160,
//     to: H160,
//     value: U256,
//     r: U256,
//     s: U256,
//     w: U256, // TODO implement getter
//     y_parity: bool,
//     gas_limit: U256,
//     gas_price: U256,
//     calldata: Vec<u8>,
//     init: Vec<u8>,
// }

// struct Eip2930Tx {
//     nonce: U256,
//     chain_id: U256,
//     from: H160,
//     to: H160,
//     value: U256,
//     r: U256,
//     s: U256,
//     // access_list: Vec<(H160, Vec<U256>)>,
//     y_parity: bool,
//     w: U256,
//     gas_limit: U256,
//     gas_price: U256,
//     calldata: Vec<u8>,
//     init: Vec<u8>,
// }

// struct Eip1559Tx {
//     nonce: U256,
//     chain_id: U256,
//     from: H160,
//     to: H160,
//     value: U256,
//     r: U256,
//     s: U256,
//     // access_list: Vec<(H160, Vec<U256>)>,
//     y_parity: bool,
//     w: U256,
//     gas_limit: U256,
//     max_fee_per_gas: U256,
//     max_priority_fee_per_gas: U256,
//     calldata: Vec<u8>,
//     init: Vec<u8>,
// }
