mod attr;
mod config;
mod css;
mod meta;
mod script;
mod strip;
mod url;

pub fn html(html: &str, page_url: &str) -> Vec<u8> {
    let proxy = config::proxy_url();
    let base = config::extract_origin(page_url);
    let full_base = config::extract_base_url(page_url);
    let mut out = script::inject_script(html, &proxy, &base);

    for a in [
        "src",
        "href",
        "poster",
        "data-src",
        "data-lazy-src",
        "data-original",
        "data-bg",
        "data-image",
        "ping",
        "formaction",
    ] {
        out = attr::rewrite_attr_full(&out, a, &base, &full_base, &proxy);
    }

    out = attr::rewrite_srcset(&out, &base, &proxy);
    out = css::rewrite_style_attr(&out, &base, &proxy);
    out = attr::rewrite_action(&out, &base, &proxy);
    out = meta::rewrite_meta_refresh(&out, &base, &proxy);
    out = meta::rewrite_meta_content_urls(&out, &base, &proxy);
    out = strip::strip_integrity(&out);
    out = strip::strip_csp(&out);
    out = css::rewrite_inline_css(&out, &base, &proxy);

    out.into_bytes()
}

pub fn css(css: &str, page_url: &str) -> Vec<u8> {
    let proxy = config::proxy_url();
    let base = config::extract_origin(page_url);
    css::rewrite_css_content(css, &base, &proxy).into_bytes()
}
