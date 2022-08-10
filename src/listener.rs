use tokio::{net::{TcpListener, TcpStream}, sync::{broadcast, mpsc}};

use crate::Db;

pub struct Listener {
    pub db : Db,
    listener: TcpListener,
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    pub fn new(listener: TcpListener, notify_shutdown: broadcast::Sender<()>, shutdown_complete_tx: mpsc::Sender<()>, shutdown_complete_rx: mpsc::Receiver<()>) -> Listener {
        Listener {
            listener,
            db: Db::new(),
            notify_shutdown,
            shutdown_complete_rx, // this is a shorthand struct initialisation
            shutdown_complete_tx: shutdown_complete_tx,
        }
    }
    pub async fn accept(&self) -> std::result::Result<TcpStream, std::io::Error> {
        match self.listener.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}