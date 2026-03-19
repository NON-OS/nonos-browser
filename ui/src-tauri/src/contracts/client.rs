use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use super::config::SOCKS_PROXY;

const SOCKS_ADDR: &str = "127.0.0.1:1080";

pub async fn verify_socks_connection() -> Result<(), String> {
    let addr: SocketAddr = SOCKS_ADDR.parse().map_err(|_| "Invalid SOCKS address")?;
    let mut stream = timeout(Duration::from_secs(3), TcpStream::connect(addr))
        .await
        .map_err(|_| "Nym connection timeout - mixnet not ready")?
        .map_err(|_| "Cannot connect to Nym SOCKS proxy - mixnet offline")?;

    stream
        .write_all(&[0x05, 0x01, 0x00])
        .await
        .map_err(|_| "SOCKS handshake failed")?;

    let mut response = [0u8; 2];
    stream
        .read_exact(&mut response)
        .await
        .map_err(|_| "SOCKS response failed")?;

    if response[0] != 0x05 {
        return Err("Invalid SOCKS version - Nym client not responding correctly".into());
    }

    Ok(())
}

pub async fn build_verified_client() -> Result<reqwest::Client, String> {
    verify_socks_connection().await?;

    let proxy =
        reqwest::Proxy::all(SOCKS_PROXY).map_err(|e| format!("Proxy config error: {}", e))?;
    reqwest::Client::builder()
        .proxy(proxy)
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Client build error: {}", e))
}

pub fn build_client() -> Result<reqwest::Client, String> {
    let proxy =
        reqwest::Proxy::all(SOCKS_PROXY).map_err(|e| format!("Proxy config error: {}", e))?;
    reqwest::Client::builder()
        .proxy(proxy)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Client build error: {}", e))
}
