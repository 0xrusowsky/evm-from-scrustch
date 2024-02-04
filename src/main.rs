// This program runs the test suite `evm.json` developed by w1nt3r.eth
// which has been borrowed from his Github repo `EVM From Scratch`.
use evm::primitives::*;
use evm::ExecutionContext;
use serde::Deserialize;

// Struct to deserialize the test inputs
#[derive(Debug, Deserialize)]
struct Evmtest {
    // Common fields for all tests
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    // Optional fields
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

// Struct to deserialize the expected test outcomes
#[derive(Debug, Deserialize)]
struct Expect {
    // Whether the transaction should be successful or not
    success: bool,
    // EVM stack after finalizing the execution of the test
    #[serde(default)]
    stack: Vec<String>,
    // EVM logs after finalizing the execution of the test
    #[serde(default)]
    logs: Vec<JsonLog>,
    // Result of executing the transaction
    #[serde(default, rename = "return", deserialize_with = "hex_string_to_bytes")]
    result: Bytes,
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

        let expected_logs: Vec<Log> = test
            .expect
            .logs
            .iter()
            .map(|l| Log::from_json(l).unwrap())
            .collect();

        let matching = result.success == test.expect.success
            && result.result == test.expect.result
            && result.stack == expected_stack
            && result.logs == expected_logs;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm.as_ref().unwrap());
            println!("]\n");

            println!("Expected result: {:?}", test.expect.result);
            println!("Actual result: {:?}", result.result);
            println!("]\n");

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for w in expected_stack {
                println!("  {:#X},", w);
            }
            println!("Expected logs: [");
            for l in expected_logs {
                println!("  {:#?},", l);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("Actual logs: [");
            for l in result.logs {
                println!("  {:#?},", l);
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
