use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn socks5_probe(addr: SocketAddr) -> Result<(), String> {
    let mut stream = timeout(Duration::from_secs(5), TcpStream::connect(addr))
        .await
        .map_err(|_| "Connection timeout")?
        .map_err(|e| format!("Connect failed: {}", e))?;

    stream
        .write_all(&[0x05, 0x01, 0x00])
        .await
        .map_err(|e| format!("Write failed: {}", e))?;

    let mut response = [0u8; 2];
    stream
        .read_exact(&mut response)
        .await
        .map_err(|e| format!("Read failed: {}", e))?;

    if response[0] != 0x05 {
        return Err("Invalid SOCKS5 version".into());
    }

    if response[1] == 0xFF {
        return Err("SOCKS5 auth rejected".into());
    }

    Ok(())
}

pub async fn wait_for_socks(addr: SocketAddr, max_attempts: u32) -> Result<(), String> {
    for i in 0..max_attempts {
        if socks5_probe(addr).await.is_ok() {
            return Ok(());
        }
        if i < max_attempts - 1 {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
    Err("SOCKS5 not ready after max attempts".into())
}
