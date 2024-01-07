use std::net::SocketAddr;

use axum::{extract::Request, Router};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::watch,
};
use tower_service::Service;
use tracing::{error, info};

use crate::services::signals::shutdown_signal;

pub async fn run(addr: impl ToSocketAddrs, app: Router) {
    let listener = TcpListener::bind(addr).await.unwrap();
    let (close_tx, close_rx) = watch::channel(());
    loop {
        let (socket, remote_addr) = tokio::select! {
            result = listener.accept() => {
                result.unwrap()
            }
            _ = shutdown_signal() => {
                info!("Shutdown signal received, no more connections will be accepted");
                break;
            }
        };

        info!({ remote_addr = ?remote_addr }, "Connection accepted");
        let tower_service = app.clone();
        let close_rx = close_rx.clone();
        tokio::spawn(handle_client(socket, tower_service, remote_addr, close_rx));
    }
    drop(close_rx);
    drop(listener);
    info!("Waiting for {} tasks to finish", close_tx.receiver_count());
    close_tx.closed().await;
}

async fn handle_client(
    socket: TcpStream,
    tower_service: Router,
    remote_addr: SocketAddr,
    close_rx: watch::Receiver<()>,
) {
    let socket = TokioIo::new(socket);
    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });
    let conn = hyper::server::conn::http1::Builder::new()
        .serve_connection(socket, hyper_service)
        .with_upgrades();
    let mut conn = std::pin::pin!(conn);

    loop {
        tokio::select! {
            result = conn.as_mut() => {
                if let Err(error) = result {
                    error!({error = ?error}, "Failed to serve connection");
                }
                break;
            }
            _ = shutdown_signal() => {
                info!("Shutdown signal received, starting graceful shutdown");
                conn.as_mut().graceful_shutdown();
            }
        }
    }
    info!(remote_addr = ?remote_addr, "Connection closed");
    drop(close_rx);
}
