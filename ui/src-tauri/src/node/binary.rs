use std::path::PathBuf;
use std::process::Command;

pub fn find_nonos_node_binary() -> Option<PathBuf> {
    let locations = [
        PathBuf::from("../../target/release/nonos-node"),
        PathBuf::from("/usr/local/bin/nonos-node"),
        PathBuf::from("/usr/bin/nonos-node"),
        dirs::data_local_dir()
            .unwrap_or_default()
            .join("nonos")
            .join("bin")
            .join("nonos-node"),
        dirs::home_dir()
            .unwrap_or_default()
            .join(".local")
            .join("bin")
            .join("nonos-node"),
    ];

    for path in locations {
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(output) = Command::new("which").arg("nonos-node").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }

    None
}
