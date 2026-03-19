use super::state::WALLET_MANAGER;
use crate::contracts::{fetch_mainnet_balances, fetch_sepolia_balances};
use crate::helpers::format_wei;
use crate::state::AppState;
use crate::types::WalletStatusResponse;
use tauri::State;

#[tauri::command]
pub async fn wallet_get_status(state: State<'_, AppState>) -> Result<WalletStatusResponse, String> {
    let mut manager = WALLET_MANAGER.write().await;

    let (initialized, locked, address) = if let Some(wallet) = manager.active() {
        (true, !wallet.is_unlocked(), Some(wallet.address().to_hex()))
    } else {
        let has_wallets = manager.has_wallets();
        (has_wallets, true, None)
    };

    drop(manager);

    let (eth_balance, nox_balance, sepolia_eth, sepolia_nox) = if let Some(ref addr) = address {
        let mainnet = fetch_mainnet_balances(addr).await.unwrap_or((0, 0));
        let sepolia = fetch_sepolia_balances(addr).await.unwrap_or((0, 0));
        (mainnet.0, mainnet.1, sepolia.0, sepolia.1)
    } else {
        (0, 0, 0, 0)
    };

    {
        let mut app_wallet = state.wallet.write().await;
        app_wallet.eth_balance = eth_balance;
        app_wallet.nox_balance = nox_balance;
        app_wallet.sepolia_eth_balance = sepolia_eth;
        app_wallet.sepolia_nox_balance = sepolia_nox;
        app_wallet.initialized = initialized;
        app_wallet.locked = locked;
        app_wallet.address = address.clone();
        app_wallet.last_refresh = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    let app_wallet = state.wallet.read().await;
    Ok(WalletStatusResponse {
        initialized,
        locked,
        address,
        nox_balance: format_wei(nox_balance),
        eth_balance: format_wei(eth_balance),
        sepolia_nox_balance: format_wei(sepolia_nox),
        sepolia_eth_balance: format_wei(sepolia_eth),
        pending_rewards: format_wei(app_wallet.pending_rewards),
    })
}
