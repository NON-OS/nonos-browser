use super::super::socks;
use super::config::{extract_origin, resolve_url};

pub async fn inline_css(html: &str, page_url: &str) -> String {
    let base = extract_origin(page_url);
    let mut result = html.to_string();

    let link_pattern = regex::Regex::new(r#"<link[^>]*rel\s*=\s*["']?stylesheet["']?[^>]*>"#).ok();
    let href_pattern = regex::Regex::new(r#"href\s*=\s*["']([^"']+)["']"#).ok();

    if let (Some(link_re), Some(href_re)) = (link_pattern, href_pattern) {
        let links: Vec<String> = link_re
            .find_iter(html)
            .map(|m| m.as_str().to_string())
            .collect();

        for link in links {
            if let Some(href_cap) = href_re.captures(&link) {
                if let Some(href) = href_cap.get(1) {
                    let css_url = resolve_url(href.as_str(), &base, page_url);

                    if let Ok(Ok(css_content)) =
                        tokio::time::timeout(std::time::Duration::from_secs(5), fetch_css(&css_url))
                            .await
                    {
                        let style_tag = format!("<style>{}</style>", css_content);
                        result = result.replace(&link, &style_tag);
                    }
                }
            }
        }
    }

    result
}

async fn fetch_css(url: &str) -> Result<String, String> {
    match socks::fetch(url).await {
        Ok((status, content_type, body)) => {
            if status == 200
                && (content_type.contains("css") || content_type.contains("text/plain"))
            {
                Ok(String::from_utf8_lossy(&body).to_string())
            } else {
                Err(format!("CSS fetch failed: {}", status))
            }
        }
        Err(e) => Err(e),
    }
}
