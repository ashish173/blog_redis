use std::{io::{LineWriter, ErrorKind}, future::Future, thread::sleep};

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}, sync::{broadcast, mpsc}};
use crate::{Db, Handler};
use bytes::BytesMut;
use crate::{helper::buffer_to_array, Command};

use crate::Listener;

// pub struct Listener {
//     db : Db,
//     listener: TcpListener,
// }

// impl Listener {
//     pub fn new(listener: TcpListener) -> Listener {
//         Listener {
//             listener: listener,
//             db: Db::new(),
//         }
//     }
// }


pub async fn run (listener: TcpListener) -> std::io::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let listener = Listener::new(listener, notify_shutdown, shutdown_complete_tx, shutdown_complete_rx);
    loop {
        // create my data, wrap it in a mutex, then add atomic reference couting
        // let my_data = Arc::new(Mutex::new(vec![1, 2, 3]));
        
        // spawn a thread that will update the values
        // a clone of our Arc will be moved into the thread
        // let thread_arc = my_data.clone();
        
        
                let mut socket= listener.accept().await?;
                println!("loop run");
                // let mut db_clone = listener.db.clone();
                let mut handle = Handler::new(socket, &listener.db, listener.notify_shutdown.subscribe());
                tokio::spawn(async move{
                    if let Err(_err) = process_method(&mut handle).await{
                        println!("Connection Error");
                    }
                    // Ok(res)
                });
            }
            // Ok(())
}
// async fn process_method (socket: &mut TcpStream, db: Db,) -> Result<(), std::io::Error>{
async fn process_method (handle: &mut Handler) -> Result<(), std::io::Error>{
    sleep(std::time::Duration::new(1, 0));
    let mut buf = BytesMut::with_capacity(1024);
    handle.stream.read_buf(&mut buf).await?;
    let attrs = buffer_to_array(&mut buf);
    let command = Command::get_command(&attrs[0]);
    let res = process_query(command, attrs,  handle).await?;
    Ok(res)
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    handle: &mut Handler
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            let result = handle.db.read(&attrs);
            // entries.lock().unwrap().get(k);
            match result {
                Ok(result) => {
                    handle.stream.write_all(&result).await?;
                }
                Err(_err) => {
                    handle.stream.write_all(b"").await?;
                }
            }

            Ok(())
        }
        Command::Set => {
            let resp = handle.db.write(&attrs);
            match resp {
                Ok(result) => {
                    handle.stream.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    handle.stream.write_all(b"").await?;
                }
            }

            Ok(())
        }
        Command::Invalid => {
            handle.stream.write_all(b"invalid command").await?;
            Err(std::io::Error::from(ErrorKind::InvalidData))
        }
    }
}
