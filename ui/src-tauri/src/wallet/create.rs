use super::state::WALLET_MANAGER;
use crate::helpers::{validate_mnemonic, validate_password};
use crate::state::AppState;
use nonos_wallet::{Wallet, WalletStorage};
use tauri::State;

#[tauri::command]
pub async fn wallet_create(state: State<'_, AppState>, password: String) -> Result<String, String> {
    validate_password(&password).map_err(|e| e.to_string())?;

    let (wallet, mnemonic, _blake3_key) = Wallet::create("Default Wallet".to_string())
        .map_err(|e| format!("Failed to create wallet: {}", e))?;

    let address = wallet.address().to_hex();

    let master_key = nonos_crypto::derive_blake3_key_from_mnemonic(&mnemonic)
        .map_err(|e| format!("Key derivation failed: {}", e))?;

    {
        let mut manager = WALLET_MANAGER.write().await;
        let storage = manager.storage()?;
        storage
            .save_wallet(wallet.metadata(), &master_key.0, &password)
            .map_err(|e| format!("Failed to save wallet: {}", e))?;
        manager.set_active(wallet);
    }

    {
        let mut app_wallet = state.wallet.write().await;
        app_wallet.initialized = true;
        app_wallet.locked = false;
        app_wallet.address = Some(address);
        app_wallet.mnemonic = Some(mnemonic.clone());
    }

    Ok(mnemonic)
}

#[tauri::command]
pub async fn wallet_import(
    state: State<'_, AppState>,
    mnemonic: String,
    password: String,
) -> Result<(), String> {
    validate_password(&password).map_err(|e| e.to_string())?;
    validate_mnemonic(&mnemonic).map_err(|e| e.to_string())?;

    let wallet = Wallet::import_from_mnemonic("Imported Wallet".to_string(), &mnemonic)
        .map_err(|e| format!("Failed to import wallet: {}", e))?;

    let address = wallet.address().to_hex();

    let master_key = nonos_crypto::derive_blake3_key_from_mnemonic(&mnemonic)
        .map_err(|e| format!("Key derivation failed: {}", e))?;

    {
        let mut manager = WALLET_MANAGER.write().await;
        let storage = manager.storage()?;
        storage
            .save_wallet(wallet.metadata(), &master_key.0, &password)
            .map_err(|e| format!("Failed to save wallet: {}", e))?;
        manager.set_active(wallet);
    }

    {
        let mut app_wallet = state.wallet.write().await;
        app_wallet.initialized = true;
        app_wallet.locked = false;
        app_wallet.address = Some(address);
        app_wallet.mnemonic = Some(mnemonic);
    }

    Ok(())
}
