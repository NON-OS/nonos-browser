mod config;
mod css;
mod handler;
mod response;
mod security;

pub use config::LOCAL_PROXY_PORT;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start_local_proxy_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], LOCAL_PROXY_PORT));
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(_) => return,
    };

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let io = TokioIo::new(stream);
            tokio::spawn(async move {
                let _ = http1::Builder::new()
                    .serve_connection(io, service_fn(handler::handle))
                    .await;
            });
        }
    }
}
