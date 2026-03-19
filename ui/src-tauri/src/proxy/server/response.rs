use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

const ALLOWED_ORIGINS: [&str; 3] = [
    "tauri://localhost",
    "https://tauri.localhost",
    "http://localhost",
];

pub fn cors_response(status: u16, body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", ALLOWED_ORIGINS[0])
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "x-nonos-session, content-type",
        )
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap()
}

pub fn error_response(status: u16, msg: &str) -> Response<Full<Bytes>> {
    let body = format!(
        r#"<!DOCTYPE html><html><head><title>Error</title></head>
<body style="font-family:sans-serif;padding:40px;background:#0a0a0f;color:#e0e0e0;">
<h1 style="color:#ff6666;">Error</h1><p>{}</p>
<p><a href="javascript:history.back()" style="color:#66ffff;">Go Back</a></p>
</body></html>"#,
        msg
    );
    Response::builder()
        .status(status)
        .header("Content-Type", "text/html")
        .header("Access-Control-Allow-Origin", ALLOWED_ORIGINS[0])
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

pub fn is_allowed_origin(origin: &str) -> bool {
    ALLOWED_ORIGINS.iter().any(|o| origin.starts_with(o))
}
