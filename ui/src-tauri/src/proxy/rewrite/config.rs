pub const LOCAL_PROXY_PORT: u16 = 9060;

pub fn proxy_url() -> String {
    format!("http://localhost:{}/proxy?url=", LOCAL_PROXY_PORT)
}

pub fn extract_origin(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .map(|u| format!("{}://{}", u.scheme(), u.host_str().unwrap_or("")))
        .unwrap_or_default()
}

pub fn extract_base_url(url: &str) -> String {
    if let Ok(u) = url::Url::parse(url) {
        let path = u.path();
        if let Some(last_slash) = path.rfind('/') {
            return format!(
                "{}://{}{}",
                u.scheme(),
                u.host_str().unwrap_or(""),
                &path[..=last_slash]
            );
        }
        return format!("{}://{}/", u.scheme(), u.host_str().unwrap_or(""));
    }
    String::new()
}
