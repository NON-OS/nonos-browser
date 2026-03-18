use super::{
    config::{MAX_REDIRECTS, SOCKS_ADDR, USER_AGENT},
    http, parse,
};
use std::future::Future;
use std::pin::Pin;
use tokio_socks::tcp::Socks5Stream;

pub async fn fetch(url: &str) -> Result<(u16, String, Vec<u8>), String> {
    fetch_inner(url.to_string(), 0).await
}

fn fetch_inner(
    url: String,
    depth: u8,
) -> Pin<Box<dyn Future<Output = Result<(u16, String, Vec<u8>), String>> + Send>> {
    Box::pin(async move {
        if depth > MAX_REDIRECTS {
            return Err("Too many redirects".to_string());
        }

        let parsed = url::Url::parse(&url).map_err(|e| e.to_string())?;
        let host = parsed.host_str().ok_or("No host")?.to_string();
        let port = parsed
            .port()
            .unwrap_or(if parsed.scheme() == "https" { 443 } else { 80 });
        let path = match parsed.query() {
            Some(q) => format!("{}?{}", parsed.path(), q),
            None => parsed.path().to_string(),
        };
        let path = if path.is_empty() {
            "/".to_string()
        } else {
            path
        };
        let scheme = parsed.scheme().to_string();

        let stream = Socks5Stream::connect(SOCKS_ADDR, (host.as_str(), port))
            .await
            .map_err(|e| format!("SOCKS5: {}", e))?;

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\nAccept-Language: en-US,en;q=0.9\r\nAccept-Encoding: gzip, deflate\r\nDNT: 1\r\nSec-Fetch-Dest: document\r\nSec-Fetch-Mode: navigate\r\nSec-Fetch-Site: none\r\nSec-Fetch-User: ?1\r\nUpgrade-Insecure-Requests: 1\r\nConnection: close\r\n\r\n",
            path, host, USER_AGENT
        );

        let (status, headers, body) = if scheme == "https" {
            http::fetch_https(stream, &host, &request).await?
        } else {
            http::fetch_http(stream, &request).await?
        };

        if matches!(status, 301 | 302 | 303 | 307 | 308) {
            if let Some(location) = parse::extract_header(&headers, "location") {
                let new_url = resolve_redirect(&location, &scheme, &host);
                return fetch_inner(new_url, depth + 1).await;
            }
        }

        let encoding = parse::extract_header(&headers, "content-encoding").unwrap_or_default();
        let body = parse::decompress(&body, &encoding)?;

        let content_type = parse::extract_header(&headers, "content-type")
            .unwrap_or_else(|| "text/html".to_string())
            .split(';')
            .next()
            .unwrap_or("text/html")
            .trim()
            .to_string();

        Ok((status, content_type, body))
    })
}

fn resolve_redirect(location: &str, scheme: &str, host: &str) -> String {
    if location.starts_with("http") {
        location.to_string()
    } else if location.starts_with("//") {
        format!("{}:{}", scheme, location)
    } else if location.starts_with('/') {
        format!("{}://{}{}", scheme, host, location)
    } else {
        format!("{}://{}/{}", scheme, host, location)
    }
}
