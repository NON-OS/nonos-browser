use http_body_util::Full;
use hyper::body::Bytes;
use hyper::Response;

pub fn cors_response(status: u16, body: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
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
        .header("Access-Control-Allow-Origin", "*")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}
