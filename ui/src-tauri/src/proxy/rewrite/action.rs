pub fn rewrite_action(html: &str, base: &str, proxy: &str) -> String {
    let mut out = html.to_string();
    for q in ['"', '\''] {
        let pattern = format!("action={}", q);
        let mut result = String::new();
        let mut rest = out.as_str();

        while let Some(pos) = rest.to_lowercase().find(&pattern) {
            result.push_str(&rest[..pos]);
            let after = &rest[pos + pattern.len()..];
            if let Some(end) = after.find(q) {
                let val = &after[..end];
                if val.starts_with("http") && !val.contains("localhost:9060") {
                    result.push_str(&format!(
                        "action={}{}{}",
                        q,
                        proxy,
                        urlencoding::encode(val)
                    ));
                } else if val.starts_with('/') && !val.starts_with("//") {
                    result.push_str(&format!(
                        "action={}{}{}{}",
                        q,
                        proxy,
                        urlencoding::encode(base),
                        urlencoding::encode(val)
                    ));
                } else {
                    result.push_str(&rest[pos..pos + pattern.len()]);
                    result.push_str(val);
                }
                rest = &after[end..];
            } else {
                result.push_str(&rest[pos..pos + pattern.len()]);
                rest = after;
            }
        }
        result.push_str(rest);
        out = result;
    }
    out
}
