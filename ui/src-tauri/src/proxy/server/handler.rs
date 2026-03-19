use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{body::Incoming, Request, Response};

use super::super::{rewrite, session, socks};
use super::{css, response, security};

pub async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    if !validate_request(&req) {
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
        return Ok(response::error_response(403, "Private network blocked"));
    }

    match socks::fetch(&url).await {
        Ok((status, content_type, body)) => {
            let final_body = process_body(&content_type, &body, &url).await;
            Ok(build_response(status, &content_type, final_body))
        }
        Err(e) => Ok(response::error_response(502, &format!("Failed: {}", e))),
    }
}

fn validate_request(req: &Request<Incoming>) -> bool {
    let token = req
        .headers()
        .get("x-nonos-session")
        .and_then(|v| v.to_str().ok());

    if let Some(t) = token {
        return session::validate_session(t);
    }

    let origin = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    origin.is_empty() || response::is_allowed_origin(origin)
}

async fn process_body(content_type: &str, body: &[u8], url: &str) -> Vec<u8> {
    if content_type.contains("text/html") {
        let html = String::from_utf8_lossy(body);
        let with_css = css::inline_css(&html, url).await;
        rewrite::html(&with_css, url)
    } else if content_type.contains("text/css") {
        let css_str = String::from_utf8_lossy(body);
        rewrite::css(&css_str, url)
    } else {
        body.to_vec()
    }
}

fn build_response(status: u16, content_type: &str, body: Vec<u8>) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Content-Type", content_type)
        .header("Access-Control-Allow-Origin", "tauri://localhost")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "x-nonos-session, content-type",
        )
        .body(Full::new(Bytes::from(body)))
        .unwrap()
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
