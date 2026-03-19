use tiny_keccak::{Hasher, Keccak};

pub fn function_selector(signature: &str) -> [u8; 4] {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(signature.as_bytes());
    keccak.finalize(&mut hash);
    [hash[0], hash[1], hash[2], hash[3]]
}

pub fn encode_address(address: &str) -> [u8; 32] {
    let mut result = [0u8; 32];
    let addr = address.trim_start_matches("0x");
    if let Ok(bytes) = hex::decode(addr) {
        let start = 32 - bytes.len().min(20);
        result[start..].copy_from_slice(&bytes[..bytes.len().min(20)]);
    }
    result
}

pub fn encode_uint256(value: u128) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[16..].copy_from_slice(&value.to_be_bytes());
    result
}

pub fn encode_approve(spender: &str, amount: u128) -> Vec<u8> {
    let mut data = Vec::with_capacity(68);
    data.extend_from_slice(&function_selector("approve(address,uint256)"));
    data.extend_from_slice(&encode_address(spender));
    data.extend_from_slice(&encode_uint256(amount));
    data
}
