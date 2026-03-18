use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{body::Incoming, Request, Response};

use super::super::{rewrite, socks};
use super::{css, response, security};

pub async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let origin = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let allowed = origin.is_empty()
        || origin.starts_with("tauri://")
        || origin.starts_with("http://localhost")
        || origin.starts_with("https://tauri.localhost");

    if !allowed {
        return Ok(response::error_response(403, "Forbidden"));
    }

    if req.method() == hyper::Method::OPTIONS {
        return Ok(response::cors_response(200, ""));
    }

    let url = match extract_url(req.uri().query()) {
        Some(u) => u,
        None => return Ok(response::error_response(400, "Missing url parameter")),
    };

    if security::is_private_url(&url) {
        return Ok(response::error_response(
            403,
            "Access to private networks blocked",
        ));
    }

    match socks::fetch(&url).await {
        Ok((status, content_type, body)) => {
            let final_body = if content_type.contains("text/html") {
                let html = String::from_utf8_lossy(&body);
                let html_with_css = css::inline_css(&html, &url).await;
                rewrite::html(&html_with_css, &url)
            } else if content_type.contains("text/css") {
                let css_str = String::from_utf8_lossy(&body);
                rewrite::css(&css_str, &url)
            } else {
                body
            };

            Ok(Response::builder()
                .status(status)
                .header("Content-Type", content_type)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .header("Access-Control-Allow-Headers", "*")
                .body(Full::new(Bytes::from(final_body)))
                .unwrap())
        }
        Err(e) => Ok(response::error_response(
            502,
            &format!("Connection failed: {}", e),
        )),
    }
}

fn extract_url(query: Option<&str>) -> Option<String> {
    query?.split('&').find_map(|p| {
        let mut parts = p.splitn(2, '=');
        if parts.next()? == "url" {
            urlencoding::decode(parts.next()?)
                .ok()
                .map(|s| s.into_owned())
        } else {
            None
        }
    })
}
