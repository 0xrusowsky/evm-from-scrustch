pub fn rlp_encode(input: &[u8]) -> Vec<u8> {
    if input.len() == 1 && input[0] < 0x80 {
        vec![input[0]]
    } else {
        let mut encoded = encode_length(input.len(), 0x80);
        encoded.extend_from_slice(input);
        encoded
    }
}

fn encode_length(length: usize, offset: u8) -> Vec<u8> {
    if length < 56 {
        vec![(length as u8) + offset]
    } else if length < (1 << 56) {
        let mut binary_length = to_binary(length);
        let mut encoded = vec![(binary_length.len() as u8) + offset + 55];
        encoded.append(&mut binary_length);
        encoded
    } else {
        panic!("RLP input too long");
    }
}

fn to_binary(mut x: usize) -> Vec<u8> {
    let mut result = Vec::new();
    while x > 0 {
        result.insert(0, (x % 256) as u8);
        x /= 256;
    }
    result
}