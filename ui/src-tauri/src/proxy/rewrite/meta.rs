pub fn rewrite_meta_refresh(html: &str, base: &str, proxy: &str) -> String {
    let mut out = html.to_string();
    let lower = out.to_lowercase();

    if let Some(start) = lower.find("http-equiv=\"refresh\"") {
        if let Some(meta_start) = lower[..start].rfind('<') {
            let meta_tag = &out[meta_start..];
            if let Some(content_start) = meta_tag.to_lowercase().find("content=\"") {
                let content_begin = meta_start + content_start + 9;
                if let Some(content_end) = out[content_begin..].find('"') {
                    let content_val = &out[content_begin..content_begin + content_end];
                    if let Some(url_pos) = content_val.to_lowercase().find("url=") {
                        let url_start = url_pos + 4;
                        let url = content_val[url_start..].trim();
                        if url.starts_with("http") && !url.contains("localhost:9060") {
                            let new_url = format!("{}{}", proxy, urlencoding::encode(url));
                            let new_content =
                                format!("{}url={}", &content_val[..url_start], new_url);
                            out = format!(
                                "{}{}{}",
                                &out[..content_begin],
                                new_content,
                                &out[content_begin + content_end..]
                            );
                        } else if url.starts_with('/') && !url.starts_with("//") {
                            let new_url = format!(
                                "{}{}{}",
                                proxy,
                                urlencoding::encode(base),
                                urlencoding::encode(url)
                            );
                            let new_content =
                                format!("{}url={}", &content_val[..url_start], new_url);
                            out = format!(
                                "{}{}{}",
                                &out[..content_begin],
                                new_content,
                                &out[content_begin + content_end..]
                            );
                        }
                    }
                }
            }
        }
    }
    out
}

pub fn rewrite_meta_content_urls(html: &str, _base: &str, proxy: &str) -> String {
    let mut result = String::new();
    let mut rest = html;
    let url_props = [
        "og:image",
        "og:url",
        "og:video",
        "og:audio",
        "twitter:image",
        "twitter:url",
        "twitter:player",
    ];

    while let Some(meta_start) = rest.to_lowercase().find("<meta") {
        result.push_str(&rest[..meta_start]);
        let meta_rest = &rest[meta_start..];

        if let Some(meta_end) = meta_rest.find('>') {
            let meta_tag = &meta_rest[..meta_end + 1];
            let meta_lower = meta_tag.to_lowercase();
            let is_url_meta = url_props.iter().any(|p| meta_lower.contains(p));

            if is_url_meta {
                if let Some(content_idx) = meta_lower.find("content=\"") {
                    let content_start = content_idx + 9;
                    let after_content = &meta_tag[content_start..];
                    if let Some(content_end) = after_content.find('"') {
                        let content_val = &after_content[..content_end];
                        let trimmed = content_val.trim();
                        if (trimmed.starts_with("http://") || trimmed.starts_with("https://"))
                            && !trimmed.contains("localhost:9060")
                        {
                            let new_val = format!("{}{}", proxy, urlencoding::encode(trimmed));
                            let new_meta = format!(
                                "{}{}{}",
                                &meta_tag[..content_start],
                                new_val,
                                &meta_tag[content_start + content_end..]
                            );
                            result.push_str(&new_meta);
                            rest = &meta_rest[meta_end + 1..];
                            continue;
                        }
                    }
                }
            }
            result.push_str(meta_tag);
            rest = &meta_rest[meta_end + 1..];
        } else {
            result.push_str(&rest[meta_start..meta_start + 5]);
            rest = &meta_rest[5..];
        }
    }
    result.push_str(rest);
    result
}
