pub fn strip_integrity(html: &str) -> String {
    let mut out = html.to_string();
    for quote in ['"', '\''] {
        let pattern = format!(" integrity={}", quote);
        while let Some(start) = out.find(&pattern) {
            if let Some(end) = out[start + pattern.len()..].find(quote) {
                out = format!(
                    "{}{}",
                    &out[..start],
                    &out[start + pattern.len() + end + 1..]
                );
            } else {
                break;
            }
        }
    }
    out
}

pub fn strip_csp(html: &str) -> String {
    let mut out = html.to_string();
    let patterns = [
        "content-security-policy",
        "x-content-security-policy",
        "x-webkit-csp",
    ];
    for pat in patterns {
        while let Some(start) = out.to_lowercase().find(&format!("http-equiv=\"{}\"", pat)) {
            if let Some(meta_start) = out[..start].rfind('<') {
                if let Some(meta_end) = out[start..].find('>') {
                    out = format!("{}{}", &out[..meta_start], &out[start + meta_end + 1..]);
                    continue;
                }
            }
            break;
        }
    }
    out
}
