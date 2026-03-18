pub fn rewrite_value(value: &str, base: &str, full_base: &str, proxy: &str) -> String {
    let v = value.trim();
    if v.is_empty()
        || v.starts_with("data:")
        || v.starts_with("blob:")
        || v.starts_with('#')
        || v.starts_with("javascript:")
        || v.contains("localhost:9060")
    {
        return value.to_string();
    }
    if v.starts_with("https://") || v.starts_with("http://") {
        return format!("{}{}", proxy, urlencoding::encode(v));
    }
    if v.starts_with("//") {
        return format!("{}https:{}", proxy, urlencoding::encode(v));
    }
    if v.starts_with('/') {
        return format!(
            "{}{}{}",
            proxy,
            urlencoding::encode(base),
            urlencoding::encode(v)
        );
    }
    format!(
        "{}{}{}",
        proxy,
        urlencoding::encode(full_base),
        urlencoding::encode(v)
    )
}
