pub fn is_private_url(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            if host == "localhost" || host.ends_with(".localhost") {
                return true;
            }
            if host.starts_with("127.") || host == "::1" || host == "[::1]" {
                return true;
            }
            if host.starts_with("10.") {
                return true;
            }
            if host.starts_with("192.168.") {
                return true;
            }
            if host.starts_with("172.") {
                if let Some(second) = host.split('.').nth(1) {
                    if let Ok(n) = second.parse::<u8>() {
                        if (16..=31).contains(&n) {
                            return true;
                        }
                    }
                }
            }
            if host.starts_with("169.254.") {
                return true;
            }
            if host == "0.0.0.0" || host.starts_with("0.") {
                return true;
            }
        }
        if parsed.scheme() == "file" {
            return true;
        }
    }
    false
}
