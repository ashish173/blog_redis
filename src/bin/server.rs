use std::{fmt::Error, io::ErrorKind};
use tokio::sync::{broadcast, mpsc};
use blog_redis:: server;
// use bytes::BytesMut;
use tokio::{
    io::AsyncReadExt,
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use blog_redis::Listener;
use tokio::signal;



#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let shutdown = signal::ctrl_c();
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut listener = Listener::new(listener, notify_shutdown, shutdown_complete_tx, shutdown_complete_rx);
    tokio::select! {
        res = server::run(&mut listener) => {
             if let Err(_err) = res {
               println!("failed to accept connection");               
            }
        }
        _ = shutdown => {
            // println!("========{:?}", shut);
            println!("inside shutdown loop");
            
        }
    }
    println!("before notify shutdown drop");
    drop(listener.notify_shutdown);
    drop(listener.shutdown_complete_tx);
    let _ = listener.shutdown_complete_rx.recv().await;

    Ok(())
}

// async fn process_method (socket: &mut TcpStream, db: Db,) -> Result<(), std::io::Error>{
//     // sleep(std::time::Duration::new(1, 0));
//     let mut buf = BytesMut::with_capacity(1024);
//     socket.read_buf(&mut buf).await?;
//     let attrs = buffer_to_array(&mut buf);
//     let command = Command::get_command(&attrs[0]);
//     let res = process_query(command, attrs,  socket,  db).await?;
//     Ok(res)
// }

// async fn process_query(
//     command: Command,
//     attrs: Vec<String>,
//     socket: &mut TcpStream,
//     db: Db,
// ) -> std::io::Result<()> {
//     match command {
//         Command::Get => {
//             let result = db.read(&attrs);
//             match result {
//                 Ok(result) => {
//                     socket.write_all(&result).await?;
//                 }
//                 Err(_err) => {
//                     socket.write_all(b"").await?;
//                 }
//             }

//             Ok(())
//         }
//         Command::Set => {
//             let resp = db.write(&attrs);
//             match resp {
//                 Ok(result) => {
//                     socket.write_all(&result.as_bytes()).await?;
//                 }
//                 Err(_err) => {
//                     socket.write_all(b"").await?;
//                 }
//             }

//             Ok(())
//         }
//         Command::Invalid => {
//             socket.write_all(b"invalid command").await?;
//             Err(std::io::Error::from(ErrorKind::InvalidData))
//         }
//     }
// }
