use super::state::WALLET_MANAGER;
use crate::state::AppState;
use nonos_wallet::{Wallet, WalletStorage};
use tauri::State;

#[tauri::command]
pub async fn wallet_unlock(state: State<'_, AppState>, password: String) -> Result<(), String> {
    let mut manager = WALLET_MANAGER.write().await;
    let storage = manager.storage()?;

    let wallets = storage
        .list_wallets()
        .map_err(|e| format!("Failed to list wallets: {}", e))?;

    if wallets.is_empty() {
        return Err("No wallet found".into());
    }

    let wallet_id = &wallets[0];
    let metadata = storage
        .load_metadata(wallet_id)
        .map_err(|e| format!("Failed to load wallet: {}", e))?;

    let master_key = storage
        .load_secrets(wallet_id, &password)
        .map_err(|_| "Wrong password".to_string())?;

    let key_hex = hex::encode(master_key);
    let wallet = Wallet::import_from_blake3_key(metadata.name.clone(), &key_hex)
        .map_err(|e| format!("Failed to unlock wallet: {}", e))?;

    let address = wallet.address().to_hex();
    manager.set_active(wallet);
    drop(manager);

    {
        let mut app_wallet = state.wallet.write().await;
        app_wallet.initialized = true;
        app_wallet.locked = false;
        app_wallet.address = Some(address);
    }

    Ok(())
}

#[tauri::command]
pub async fn wallet_lock(state: State<'_, AppState>) -> Result<(), String> {
    {
        let mut manager = WALLET_MANAGER.write().await;
        manager.clear_active();
    }

    {
        let mut app_wallet = state.wallet.write().await;
        app_wallet.locked = true;
        app_wallet.private_key = None;
        app_wallet.mnemonic = None;
    }

    Ok(())
}

#[tauri::command]
pub async fn wallet_change_password(
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    if new_password.len() < 8 {
        return Err("New password must be at least 8 characters".into());
    }

    let mut manager = WALLET_MANAGER.write().await;
    let storage = manager.storage()?;

    let wallets = storage
        .list_wallets()
        .map_err(|e| format!("Failed to list wallets: {}", e))?;

    if wallets.is_empty() {
        return Err("No wallet found".into());
    }

    let wallet_id = &wallets[0];
    storage
        .change_password(wallet_id, &old_password, &new_password)
        .map_err(|_| "Wrong password".to_string())?;

    Ok(())
}
