use super::download::download_binary;
use super::types::{get_binary_name, get_platform_binary};
use super::verify::verify_checksum;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn find_nym_binary() -> Result<PathBuf, String> {
    if let Some(path) = find_bundled().await {
        return Ok(path);
    }
    if let Some(path) = find_installed().await {
        return Ok(path);
    }
    if let Some(path) = find_in_path().await {
        return Ok(path);
    }
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nonos/bin");
    download_binary(&dir).await
}

async fn find_bundled() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let binary_name = get_binary_name();
    let target_triple = get_target_triple();
    let candidates = [
        exe_dir.join(format!("{}-{}", binary_name, target_triple)),
        exe_dir.join(binary_name),
        exe_dir.join("../Resources").join(binary_name),
    ];
    for path in candidates {
        if path.exists() {
            return Some(path);
        }
    }
    None
}

async fn find_installed() -> Option<PathBuf> {
    let path = dirs::data_dir()?.join("nonos/bin").join(get_binary_name());
    if !path.exists() {
        return None;
    }
    if let Ok(platform) = get_platform_binary() {
        if verify_checksum(&path, platform.sha256).await.is_ok() {
            return Some(path);
        }
    }
    Some(path)
}

async fn find_in_path() -> Option<PathBuf> {
    let cmd = if cfg!(windows) { "where" } else { "which" };
    let output = Command::new(cmd)
        .arg(get_binary_name())
        .output()
        .await
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path_str = String::from_utf8_lossy(&output.stdout);
    let path = PathBuf::from(path_str.lines().next()?.trim());
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn get_target_triple() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        _ => "unknown",
    }
}
