pub fn rewrite_css_content(css: &str, base: &str, proxy: &str) -> String {
    let mut out = css.to_string();
    out = out.replace("url(\"https://", &format!("url(\"{}https://", proxy));
    out = out.replace("url('https://", &format!("url('{}https://", proxy));
    out = out.replace("url(https://", &format!("url({}https://", proxy));
    out = out.replace("url(\"http://", &format!("url(\"{}http://", proxy));
    out = out.replace("url('http://", &format!("url('{}http://", proxy));
    out = out.replace("url(http://", &format!("url({}http://", proxy));
    out = out.replace("url(\"//", &format!("url(\"{}https://", proxy));
    out = out.replace("url('//", &format!("url('{}https://", proxy));
    out = out.replace("url(//", &format!("url({}https://", proxy));
    out = out.replace(
        "url(\"/",
        &format!("url(\"{}{}/", proxy, urlencoding::encode(base)),
    );
    out = out.replace(
        "url('/",
        &format!("url('{}{}/", proxy, urlencoding::encode(base)),
    );
    out = out.replace(
        "url(/",
        &format!("url({}{}/", proxy, urlencoding::encode(base)),
    );
    out = out.replace(
        "@import \"https://",
        &format!("@import \"{}https://", proxy),
    );
    out = out.replace("@import 'https://", &format!("@import '{}https://", proxy));
    out = out.replace(
        "@import url(\"https://",
        &format!("@import url(\"{}https://", proxy),
    );
    out
}

pub fn rewrite_inline_css(html: &str, base: &str, proxy: &str) -> String {
    let mut result = String::new();
    let mut rest = html;

    while let Some(start) = rest.to_lowercase().find("<style") {
        let after_tag = &rest[start..];
        if let Some(tag_end) = after_tag.find('>') {
            result.push_str(&rest[..start + tag_end + 1]);
            let content_start = &rest[start + tag_end + 1..];
            if let Some(close) = content_start.to_lowercase().find("</style") {
                let css_content = &content_start[..close];
                let rewritten = rewrite_css_content(css_content, base, proxy);
                result.push_str(&rewritten);
                rest = &content_start[close..];
            } else {
                rest = content_start;
            }
        } else {
            result.push_str(&rest[..start + 6]);
            rest = &rest[start + 6..];
        }
    }
    result.push_str(rest);
    result
}

pub fn rewrite_style_attr(html: &str, base: &str, proxy: &str) -> String {
    let mut result = String::new();
    let mut rest = html;

    while let Some(pos) = rest.to_lowercase().find("style=") {
        result.push_str(&rest[..pos]);
        let after_eq = &rest[pos + 6..];
        let quote = after_eq.chars().next().unwrap_or('"');
        if quote != '"' && quote != '\'' {
            result.push_str(&rest[pos..pos + 7]);
            rest = &rest[pos + 7..];
            continue;
        }
        let inner = &after_eq[1..];
        if let Some(end) = inner.find(quote) {
            let style_val = &inner[..end];
            let new_style = rewrite_css_content(style_val, base, proxy);
            result.push_str("style=");
            result.push(quote);
            result.push_str(&new_style);
            result.push(quote);
            rest = &inner[end + 1..];
        } else {
            result.push_str(&rest[pos..pos + 7]);
            rest = &rest[pos + 7..];
        }
    }
    result.push_str(rest);
    result
}
