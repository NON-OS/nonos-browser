use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub async fn verify_checksum(path: &PathBuf, expected: &str) -> Result<(), String> {
    if expected.is_empty() {
        return Err("Checksum cannot be empty".into());
    }

    let data = tokio::fs::read(path)
        .await
        .map_err(|e| format!("Read failed: {}", e))?;

    let actual = compute_sha256(&data);
    if actual != expected {
        tokio::fs::remove_file(path).await.ok();
        return Err(format!(
            "Checksum mismatch: expected {}, got {}",
            expected, actual
        ));
    }
    Ok(())
}
