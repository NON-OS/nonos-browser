use std::path::PathBuf;
use tokio::process::Command;

pub async fn find_nym_binary() -> Result<PathBuf, String> {
    let is_windows = std::env::consts::OS == "windows";
    let binary_name = if is_windows {
        "nym-socks5-client.exe"
    } else {
        "nym-socks5-client"
    };

    let mut candidates = vec![
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join(binary_name)))
            .unwrap_or_default(),
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nonos")
            .join("bin")
            .join(binary_name),
    ];

    if is_windows {
        candidates.extend(vec![
            PathBuf::from(format!("C:\\Program Files\\nym\\{}", binary_name)),
            PathBuf::from(format!(".\\{}", binary_name)),
            dirs::home_dir()
                .map(|h| {
                    h.join("AppData")
                        .join("Local")
                        .join("nym")
                        .join(binary_name)
                })
                .unwrap_or_default(),
        ]);
    } else {
        candidates.extend(vec![
            PathBuf::from("/usr/bin/nym-socks5-client"),
            PathBuf::from("/usr/local/bin/nym-socks5-client"),
            PathBuf::from("/opt/nym/bin/nym-socks5-client"),
            PathBuf::from("/opt/homebrew/bin/nym-socks5-client"),
            dirs::home_dir()
                .map(|h| h.join(".local/bin/nym-socks5-client"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|h| h.join(".nym/bin/nym-socks5-client"))
                .unwrap_or_default(),
            PathBuf::from("./nym-socks5-client"),
        ]);
    }

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    let which_cmd = if is_windows { "where" } else { "which" };
    if let Ok(output) = Command::new(which_cmd).arg(binary_name).output().await {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let path = PathBuf::from(path_str.lines().next().unwrap_or("").trim());
            if path.exists() {
                return Ok(path);
            }
        }
    }

    let download_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nonos")
        .join("bin");
    download_binary(&download_dir).await
}

async fn download_binary(target_dir: &PathBuf) -> Result<PathBuf, String> {
    let (os, arch) = (std::env::consts::OS, std::env::consts::ARCH);

    let download_url = match (os, arch) {
        ("macos", "aarch64") => "https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.13-magura/nym-socks5-client-darwin-arm64",
        ("macos", "x86_64") => "https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.13-magura/nym-socks5-client-darwin-amd64",
        ("linux", "x86_64") => "https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.13-magura/nym-socks5-client-linux-amd64",
        ("linux", "aarch64") => "https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.13-magura/nym-socks5-client-linux-arm64",
        ("windows", "x86_64") => "https://github.com/nymtech/nym/releases/download/nym-binaries-v2024.13-magura/nym-socks5-client-windows-amd64.exe",
        _ => return Err(format!("Unsupported platform: {}-{}", os, arch)),
    };

    tokio::fs::create_dir_all(target_dir)
        .await
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    let binary_name = if os == "windows" {
        "nym-socks5-client.exe"
    } else {
        "nym-socks5-client"
    };
    let target_path = target_dir.join(binary_name);

    let response = reqwest::get(download_url)
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;
    if !response.status().is_success() {
        return Err(format!("Download failed: {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download: {}", e))?;
    tokio::fs::write(&target_path, &bytes)
        .await
        .map_err(|e| format!("Failed to write binary: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&target_path)
            .map_err(|e| format!("Failed to get permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&target_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    if target_path.exists() {
        Ok(target_path)
    } else {
        Err("Failed to install nym-socks5-client binary".into())
    }
}
