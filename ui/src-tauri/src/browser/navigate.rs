use super::state::get_browser_windows;
use crate::proxy::LOCAL_PROXY_PORT;
use crate::state::{AppState, ConnectionStatus};
use std::sync::atomic::Ordering;
use tauri::{State, Window};

#[tauri::command]
pub async fn browser_navigate(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    url: String,
    window: Window,
) -> Result<String, String> {
    let target_url = if url.starts_with("http://") || url.starts_with("https://") {
        url.clone()
    } else if url.contains('.') {
        format!("https://{}", url)
    } else {
        format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(&url)
        )
    };

    {
        state.browser.write().await.history.push(target_url.clone());
    }

    let network = state.network.read().await;
    let socks_addr = network.socks_addr;
    let is_connected = matches!(network.status, ConnectionStatus::Connected);
    drop(network);

    {
        state
            .nodes
            .read()
            .await
            .total_requests
            .fetch_add(1, Ordering::Relaxed);
    }

    let tab_id = {
        let mut browser = state.browser.write().await;
        browser.next_tab_id += 1;
        browser.next_tab_id
    };
    let window_label = format!("browser-{}", tab_id);
    let browser_url = if is_connected {
        format!(
            "http://localhost:{}/proxy?url={}",
            LOCAL_PROXY_PORT,
            urlencoding::encode(&target_url)
        )
    } else {
        target_url.clone()
    };

    let _browser_window = tauri::WindowBuilder::new(
        &app_handle,
        &window_label,
        tauri::WindowUrl::External(
            browser_url
                .parse()
                .map_err(|e| format!("Invalid URL: {}", e))?,
        ),
    )
    .title(format!(
        "NONOS - {}",
        if is_connected { "Secure" } else { "Direct" }
    ))
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .center()
    .visible(true)
    .build()
    .map_err(|e| format!("Failed to create browser window: {}", e))?;

    {
        get_browser_windows()
            .lock()
            .unwrap()
            .insert(tab_id, target_url.clone());
    }

    window.emit("nonos://tab-opened", serde_json::json!({ "tab_id": tab_id, "url": target_url, "secure": is_connected, "socks_proxy": if is_connected { Some(socks_addr.to_string()) } else { None } })).ok();

    Ok(format!(
        "Opened {} in tab {} {}",
        target_url,
        tab_id,
        if is_connected {
            format!("(via Anyone Network SOCKS5: {})", socks_addr)
        } else {
            "(direct connection)".to_string()
        }
    ))
}
