use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_selected_network(state: State<'_, AppState>) -> String {
    match state.get_selected_network() {
        crate::state::SelectedNetwork::Mainnet => "mainnet".to_string(),
        crate::state::SelectedNetwork::Sepolia => "sepolia".to_string(),
    }
}

#[tauri::command]
pub fn set_selected_network(state: State<'_, AppState>, network: String) -> Result<String, String> {
    let selected = match network.to_lowercase().as_str() {
        "mainnet" => crate::state::SelectedNetwork::Mainnet,
        "sepolia" => crate::state::SelectedNetwork::Sepolia,
        _ => return Err("Invalid network. Use 'mainnet' or 'sepolia'.".into()),
    };
    state.set_selected_network(selected);
    Ok(network.to_lowercase())
}
