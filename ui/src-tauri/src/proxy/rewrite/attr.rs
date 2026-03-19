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
