/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */
use ethereum_types::U256;
use evm::types::{Bytes, Bytes32};
use evm::{Block, Call, Code, ExecutionContext, State};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    // common fields for all tests
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    // optional fields
    tx: Option<Call>,
    block: Option<Block>,
    state: Option<State>,
}

impl Evmtest {
    fn call(&self) -> Call {
        match &self.tx {
            Some(tx) => tx.clone(),
            None => Call::default(),
        }
    }

    fn block(&self) -> Block {
        match &self.block {
            Some(block) => block.clone(),
            None => Block::default(),
        }
    }

    fn state(&self) -> State {
        match &self.state {
            Some(state) => state.clone(),
            None => State::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Expect {
    success: bool,
    #[serde(default)]
    stack: Vec<String>,
    // #[serde(rename = "return")]
    // #[serde(default)]
    // ret: String,
}

fn main() {
    let text = std::fs::read_to_string("./evm.json").unwrap();
    let mut data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter_mut().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code = Bytes::from_vec(hex::decode(&test.code.bin).unwrap());
        let mut evm = ExecutionContext::new(test.call(), test.block(), test.state(), code);
        let result = evm.run();

        let expected_stack: Vec<Bytes32> = test
            .expect
            .stack
            .iter()
            .map(|v| Bytes32::from_u256(U256::from_str_radix(v, 16).unwrap()))
            .collect();

        let matching = result.success == test.expect.success && result.stack == expected_stack;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm.as_ref().unwrap());

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS\n");
    }
    println!("Congratulations!");
}
