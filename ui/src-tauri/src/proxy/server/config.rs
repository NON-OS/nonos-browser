pub const LOCAL_PROXY_PORT: u16 = 9060;

pub fn extract_origin(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .map(|u| format!("{}://{}", u.scheme(), u.host_str().unwrap_or("")))
        .unwrap_or_default()
}

pub fn resolve_url(href: &str, base: &str, page_url: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        href.to_string()
    } else if href.starts_with("//") {
        format!("https:{}", href)
    } else if href.starts_with('/') {
        format!("{}{}", base, href)
    } else {
        if let Ok(base_url) = url::Url::parse(page_url) {
            if let Ok(resolved) = base_url.join(href) {
                return resolved.to_string();
            }
        }
        format!("{}/{}", base, href)
    }
}
