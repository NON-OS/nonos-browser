use super::state::get_browser_windows;
use crate::proxy::LOCAL_PROXY_PORT;
use crate::state::AppState;
use tauri::{Manager, State};

#[tauri::command]
pub async fn browser_close_tab(app_handle: tauri::AppHandle, tab_id: u32) -> Result<(), String> {
    let window_label = format!("browser-{}", tab_id);
    if let Some(window) = app_handle.get_webview_window(&window_label) {
        window.close().map_err(|e| e.to_string())?;
    }
    get_browser_windows().lock().unwrap().remove(&tab_id);
    Ok(())
}

#[tauri::command]
pub async fn browser_get_tabs() -> Result<Vec<serde_json::Value>, String> {
    let windows = get_browser_windows().lock().unwrap();
    let tabs: Vec<_> = windows
        .iter()
        .map(|(id, url)| serde_json::json!({ "id": id, "url": url }))
        .collect();
    Ok(tabs)
}

#[tauri::command]
pub async fn browser_get_socks_proxy(state: State<'_, AppState>) -> Result<String, String> {
    let network = state.network.read().await;
    Ok(network.socks_addr.to_string())
}

#[tauri::command]
pub fn get_proxy_url(target_url: String) -> String {
    format!(
        "http://localhost:{}/proxy?url={}",
        LOCAL_PROXY_PORT,
        urlencoding::encode(&target_url)
    )
}
