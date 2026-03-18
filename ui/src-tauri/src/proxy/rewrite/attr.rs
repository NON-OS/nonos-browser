use super::url::rewrite_value;

pub fn rewrite_attr_full(
    html: &str,
    attr: &str,
    base: &str,
    full_base: &str,
    proxy: &str,
) -> String {
    let mut out = html.to_string();
    for q in ['"', '\''] {
        out = rewrite_single_attr(&out, attr, q, base, full_base, proxy);
    }
    out
}

fn rewrite_single_attr(
    html: &str,
    attr: &str,
    quote: char,
    base: &str,
    full_base: &str,
    proxy: &str,
) -> String {
    let pattern = format!("{}={}", attr, quote);
    let mut result = String::new();
    let mut rest = html;

    while let Some(pos) = rest.to_lowercase().find(&pattern.to_lowercase()) {
        result.push_str(&rest[..pos]);
        let attr_start = pos + pattern.len();
        let after = &rest[attr_start..];

        if let Some(end) = after.find(quote) {
            let value = &after[..end];
            let new_value = rewrite_value(value, base, full_base, proxy);
            result.push_str(&rest[pos..pos + pattern.len()]);
            result.push_str(&new_value);
            rest = &rest[attr_start + end..];
        } else {
            result.push_str(&rest[pos..pos + pattern.len()]);
            rest = &rest[attr_start..];
        }
    }
    result.push_str(rest);
    result
}

pub fn rewrite_srcset(html: &str, base: &str, proxy: &str) -> String {
    let mut result = String::new();
    let mut rest = html;

    while let Some(pos) = rest.to_lowercase().find("srcset=") {
        result.push_str(&rest[..pos]);
        let after_eq = &rest[pos + 7..];
        let quote = after_eq.chars().next().unwrap_or('"');
        if quote != '"' && quote != '\'' {
            result.push_str(&rest[pos..pos + 8]);
            rest = &rest[pos + 8..];
            continue;
        }
        let inner = &after_eq[1..];
        if let Some(end) = inner.find(quote) {
            let srcset_val = &inner[..end];
            let new_srcset = rewrite_srcset_value(srcset_val, base, proxy);
            result.push_str("srcset=");
            result.push(quote);
            result.push_str(&new_srcset);
            result.push(quote);
            rest = &inner[end + 1..];
        } else {
            result.push_str(&rest[pos..pos + 8]);
            rest = &rest[pos + 8..];
        }
    }
    result.push_str(rest);
    result
}

fn rewrite_srcset_value(srcset: &str, base: &str, proxy: &str) -> String {
    srcset
        .split(',')
        .map(|part| {
            let trimmed = part.trim();
            let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            if parts.is_empty() {
                return trimmed.to_string();
            }
            let url = parts[0];
            let descriptor = if parts.len() > 1 { parts[1].trim() } else { "" };
            let new_url = if url.starts_with("http") {
                format!("{}{}", proxy, urlencoding::encode(url))
            } else if url.starts_with("//") {
                format!("{}https:{}", proxy, urlencoding::encode(url))
            } else if url.starts_with('/') {
                format!(
                    "{}{}{}",
                    proxy,
                    urlencoding::encode(base),
                    urlencoding::encode(url)
                )
            } else {
                url.to_string()
            };
            if descriptor.is_empty() {
                new_url
            } else {
                format!("{} {}", new_url, descriptor)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

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
