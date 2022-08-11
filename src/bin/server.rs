use std::future::Future;

use blog_redis::server;
use blog_redis::Listener;
use tokio::signal;
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let shutdown = signal::ctrl_c();
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut listener = Listener::new(
        listener,
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
    );

    tokio::select! {
        res = server::run(&mut listener) => {
             if let Err(_err) = res {
               println!("failed to accept connection");
            }
        }
        _ = shutdown => {
            println!("inside shutdown loop");
        }
    }

    // drop(listener.notify_shutdown);
    // drop(listener.shutdown_complete_tx);
    // println!("before final shutdown");
    // let _ = listener.shutdown_complete_rx.recv().await;
    println!("after final shutdown");

    Ok(())
}
