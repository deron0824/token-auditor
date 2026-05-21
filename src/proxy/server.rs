use anyhow::Result;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

use super::handler::ProxyHandler;

/// 启动 HTTP 代理服务器
pub async fn start_proxy(listen_addr: &str, handler: Arc<ProxyHandler>) -> Result<()> {
    let addr: SocketAddr = listen_addr.parse()?;
    let listener = TcpListener::bind(addr).await?;

    info!("代理服务器已启动，监听地址: {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let handler_clone = handler.clone();

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(
                    stream,
                    service_fn(move |req| {
                        let handler = handler_clone.clone();
                        async move { handler.handle_request(req).await }
                    }),
                )
                .with_upgrades()
                .await
            {
                error!("处理连接时出错: {}", err);
            }
        });
    }
}
