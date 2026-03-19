use super::types::{get_binary_name, get_platform_binary, NYM_VERSION};
use super::verify::verify_checksum;
use std::path::PathBuf;

pub async fn download_binary(target_dir: &PathBuf) -> Result<PathBuf, String> {
    let platform = get_platform_binary()?;

    tokio::fs::create_dir_all(target_dir)
        .await
        .map_err(|e| format!("Directory creation failed: {}", e))?;

    let url = format!(
        "https://github.com/nymtech/nym/releases/download/{}/{}",
        NYM_VERSION, platform.filename
    );

    let target_path = target_dir.join(get_binary_name());
    let temp_path = target_dir.join(format!("{}.tmp", get_binary_name()));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download returned {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Read failed: {}", e))?;
    if bytes.is_empty() {
        return Err("Downloaded file is empty".into());
    }

    tokio::fs::write(&temp_path, &bytes)
        .await
        .map_err(|e| format!("Write failed: {}", e))?;

    verify_checksum(&temp_path, platform.sha256).await?;

    tokio::fs::rename(&temp_path, &target_path)
        .await
        .map_err(|e| format!("Rename failed: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        tokio::fs::set_permissions(&target_path, perms)
            .await
            .map_err(|e| format!("Chmod failed: {}", e))?;
    }

    Ok(target_path)
}
