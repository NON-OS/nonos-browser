#[derive(Debug, Default)]
pub struct WalletState {
    pub initialized: bool,
    pub locked: bool,
    pub address: Option<String>,
    pub private_key: Option<String>,
    pub mnemonic: Option<String>,
    pub nox_balance: u128,
    pub eth_balance: u128,
    pub sepolia_nox_balance: u128,
    pub sepolia_eth_balance: u128,
    pub pending_rewards: u128,
    pub staked_amount: u128,
    pub current_epoch: u64,
    pub last_refresh: u64,
}
