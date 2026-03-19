use super::abi_core::{encode_address, encode_uint256, function_selector};

pub fn encode_lock(amount: u128, tier: u8) -> Vec<u8> {
    let mut data = Vec::with_capacity(68);
    data.extend_from_slice(&function_selector("lock(uint256,uint8)"));
    data.extend_from_slice(&encode_uint256(amount));
    data.extend_from_slice(&encode_uint256(tier as u128));
    data
}

pub fn encode_unlock(lock_id: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("unlock(uint256)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data
}

pub fn encode_early_unlock(lock_id: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("earlyUnlock(uint256)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data
}

pub fn encode_extend_lock(lock_id: u64, new_tier: u8) -> Vec<u8> {
    let mut data = Vec::with_capacity(68);
    data.extend_from_slice(&function_selector("extendLock(uint256,uint8)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data.extend_from_slice(&encode_uint256(new_tier as u128));
    data
}

pub fn encode_claim_lock_rewards(lock_id: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("claimRewards(uint256)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data
}

pub fn encode_claim_all() -> Vec<u8> {
    function_selector("claimAll()").to_vec()
}

pub fn encode_compound(lock_id: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("compound(uint256)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data
}

pub fn encode_get_locks(address: &str) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("getLocks(address)"));
    data.extend_from_slice(&encode_address(address));
    data
}

pub fn encode_pending_rewards(lock_id: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(36);
    data.extend_from_slice(&function_selector("pendingRewards(uint256)"));
    data.extend_from_slice(&encode_uint256(lock_id as u128));
    data
}

pub fn encode_total_locked() -> Vec<u8> {
    function_selector("totalLocked()").to_vec()
}

pub fn encode_weighted_total() -> Vec<u8> {
    function_selector("weightedTotal()").to_vec()
}

pub fn encode_current_epoch() -> Vec<u8> {
    function_selector("currentEpoch()").to_vec()
}

pub fn encode_epoch_pool() -> Vec<u8> {
    function_selector("epochPool()").to_vec()
}
