use serde::Serialize;

use super::{
    FEE_SWAP_SEPOLIA, NODE_OPERATOR_REWARDS_SEPOLIA, NOX_STAKING_POOL_SEPOLIA,
    NOX_TOKEN_ADDRESS_MAINNET, NOX_TOKEN_ADDRESS_SEPOLIA, PRIVACY_LIQUIDITY_POOL_SEPOLIA,
    PRIVACY_WORK_ECONOMY_SEPOLIA, ZEROSTATE_PASS_MAINNET, ZEROSTATE_STAKING_SEPOLIA,
};

#[derive(Serialize)]
pub struct ContractAddresses {
    pub nox_token_mainnet: &'static str,
    pub nox_token_sepolia: &'static str,
    pub nox_staking_pool_sepolia: &'static str,
    pub privacy_liquidity_pool_sepolia: &'static str,
    pub privacy_work_economy_sepolia: &'static str,
    pub node_operator_rewards_sepolia: &'static str,
    pub zerostate_staking_sepolia: &'static str,
    pub fee_swap_sepolia: &'static str,
    pub zerostate_pass_mainnet: &'static str,
}

#[tauri::command]
pub fn get_contract_addresses() -> ContractAddresses {
    ContractAddresses {
        nox_token_mainnet: NOX_TOKEN_ADDRESS_MAINNET,
        nox_token_sepolia: NOX_TOKEN_ADDRESS_SEPOLIA,
        nox_staking_pool_sepolia: NOX_STAKING_POOL_SEPOLIA,
        privacy_liquidity_pool_sepolia: PRIVACY_LIQUIDITY_POOL_SEPOLIA,
        privacy_work_economy_sepolia: PRIVACY_WORK_ECONOMY_SEPOLIA,
        node_operator_rewards_sepolia: NODE_OPERATOR_REWARDS_SEPOLIA,
        zerostate_staking_sepolia: ZEROSTATE_STAKING_SEPOLIA,
        fee_swap_sepolia: FEE_SWAP_SEPOLIA,
        zerostate_pass_mainnet: ZEROSTATE_PASS_MAINNET,
    }
}
