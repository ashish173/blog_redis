use std::io::ErrorKind;

use crate::{helper::buffer_to_array, Command};
use crate::{Db, Listener};
use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::{broadcast, mpsc},
};

pub struct Handler {
    pub connection: Connection,
    pub db: Db,
    pub shutdown: Shutdown,
    // when the handler object is dropped, this sends a message to the receiver
    _shutdown_complete: mpsc::Sender<()>,
}

pub struct Connection {
    pub stream: TcpStream,
}

pub struct Shutdown {
    shutdown: bool,
    notify: broadcast::Receiver<()>,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection { stream: stream }
    }

    pub async fn read_buf_data(&mut self) -> Option<(Command, Vec<String>)> {
        let mut buf = BytesMut::with_capacity(1024);
        match self.stream.read_buf(&mut buf).await {
            Ok(size) => {
                if size == 0 {
                    // returning from empty buffer
                    return None;
                }
            }
            Err(err) => {
                println!("error {:?}", err);
                return None;
            }
        };
        let attrs = buffer_to_array(&mut buf);
        Some((Command::get_command(&attrs[0]), attrs))
    }
}

impl Shutdown {
    fn new(shutdown: bool, notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown { shutdown, notify }
    }

    pub async fn listen_recv(&mut self) -> Result<(), tokio::sync::broadcast::error::RecvError> {
        self.notify.recv().await?; // returns error of type `tokio::sync::broadcast::error::RecvError`
        self.shutdown = true;
        Ok(())
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }
}

impl Handler {
    pub fn new(listener: &Listener, socket: TcpStream) -> Handler {
        Handler {
            connection: Connection::new(socket),
            db: listener.db.clone(),
            shutdown: Shutdown::new(false, listener.notify_shutdown.subscribe()),
            _shutdown_complete: listener.shutdown_complete_tx.clone(),
        }
    }

    pub async fn process_query(
        &mut self,
        command: Command,
        attrs: Vec<String>,
    ) -> Result<(), std::io::Error> {
        let connection = &mut self.connection;
        let db = &self.db;

        match command {
            Command::Get => {
                let result = db.read(&attrs);
                match result {
                    Ok(result) => {
                        connection.stream.write_all(&result).await?;
                    }
                    Err(_err) => {
                        connection.stream.write_all(b"").await?;
                    }
                }

                return Ok(());
            }
            Command::Set => {
                let resp = db.write(&attrs);
                match resp {
                    Ok(result) => {
                        connection.stream.write_all(&result.as_bytes()).await?;
                    }
                    Err(_err) => {
                        connection.stream.write_all(b"").await?;
                    }
                }

                return Ok(());
            }
            Command::Invalid => {
                connection.stream.write_all(b"invalid command").await?;
                Err(std::io::Error::from(ErrorKind::InvalidData))
            }
        }
    }
}
