use super::abi_core::{encode_uint256, function_selector};

pub fn encode_stake(amount: u128) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("stake(uint256)"));
    data.extend_from_slice(&encode_uint256(amount));
    data
}

pub fn encode_unstake(amount: u128) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("unstake(uint256)"));
    data.extend_from_slice(&encode_uint256(amount));
    data
}

pub fn encode_claim() -> Vec<u8> {
    function_selector("claim()").to_vec()
}
