use super::parse::parse_response;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_socks::tcp::Socks5Stream;

pub async fn fetch_https(
    stream: Socks5Stream<tokio::net::TcpStream>,
    host: &str,
    request: &str,
) -> Result<(u16, String, Vec<u8>), String> {
    let connector = tokio_native_tls::TlsConnector::from(
        native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(false)
            .build()
            .map_err(|e| e.to_string())?,
    );

    let mut tls = connector
        .connect(host, stream.into_inner())
        .await
        .map_err(|e| format!("TLS handshake failed: {}", e))?;

    tls.write_all(request.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    tls.flush().await.map_err(|e| e.to_string())?;

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf).await.map_err(|e| e.to_string())?;
    parse_response(&buf)
}

pub async fn fetch_http(
    stream: Socks5Stream<tokio::net::TcpStream>,
    request: &str,
) -> Result<(u16, String, Vec<u8>), String> {
    let mut tcp = stream.into_inner();
    tcp.write_all(request.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    tcp.flush().await.map_err(|e| e.to_string())?;

    let mut buf = Vec::new();
    tcp.read_to_end(&mut buf).await.map_err(|e| e.to_string())?;
    parse_response(&buf)
}
