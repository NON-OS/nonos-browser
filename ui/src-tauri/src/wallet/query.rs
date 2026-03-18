use super::state::WALLET_MANAGER;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn wallet_get_address(_state: State<'_, AppState>) -> Result<Option<String>, String> {
    let manager = WALLET_MANAGER.read().await;
    Ok(manager.active().map(|w| w.address().to_hex()))
}

#[tauri::command]
pub async fn wallet_get_transactions(
    _state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not initialized")?;

    let txs: Vec<serde_json::Value> = wallet
        .transactions()
        .iter()
        .map(|tx| {
            serde_json::json!({
                "hash": tx.hash.to_hex(),
                "status": format!("{:?}", tx.status),
            })
        })
        .collect();

    Ok(txs)
}

#[tauri::command]
pub async fn wallet_check_exists() -> Result<bool, String> {
    let mut manager = WALLET_MANAGER.write().await;
    Ok(manager.has_wallets())
}

#[tauri::command]
pub async fn wallet_get_stealth_address() -> Result<String, String> {
    let manager = WALLET_MANAGER.read().await;
    let wallet = manager.active().ok_or("Wallet not unlocked")?;

    wallet
        .generate_receive_stealth_address()
        .map_err(|e| format!("Failed to generate stealth address: {}", e))
}
