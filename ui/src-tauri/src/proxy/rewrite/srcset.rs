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
            let new_url = rewrite_srcset_url(url, base, proxy);
            if descriptor.is_empty() {
                new_url
            } else {
                format!("{} {}", new_url, descriptor)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn rewrite_srcset_url(url: &str, base: &str, proxy: &str) -> String {
    if url.starts_with("http") {
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
    }
}
