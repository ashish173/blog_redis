use tokio::{net::TcpStream, sync::{broadcast, mpsc}, stream};
use crate::Db;

pub struct Handler {
    pub stream: TcpStream,
    pub db: Db,
    pub notify: broadcast::Receiver<()>,
    pub shutdown: bool,
    // _shutdown_complete: mpsc::Sender<()>,
}

impl Handler {
    pub fn new(stream:TcpStream, db: &Db, notify: broadcast::Receiver<()>) -> Handler {
        Handler {
            stream,
            db: db.clone(),
            notify,
            shutdown: false,

        }
    }

    pub async fn listen_recv(&mut self) {
        if self.shutdown {
            return;

        }
        println!("inside Listen_recv");
        self.notify.recv().await;
        self.shutdown = true;
    } 
    
}