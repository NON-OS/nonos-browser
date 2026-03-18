use super::types::ConnectionStatus;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct NetworkState {
    pub status: ConnectionStatus,
    pub bootstrap_progress: u8,
    pub circuits: u32,
    pub socks_addr: SocketAddr,
    pub error: Option<String>,
    pub nym_pid: Option<u32>,
    pub data_dir: PathBuf,
}

impl Default for NetworkState {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nonos")
            .join("nym");

        Self {
            status: ConnectionStatus::Disconnected,
            bootstrap_progress: 0,
            circuits: 0,
            socks_addr: SocketAddr::from(([127, 0, 0, 1], 1080)),
            error: None,
            nym_pid: None,
            data_dir,
        }
    }
}
