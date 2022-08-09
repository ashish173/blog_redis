use std::{fmt::Error, io::ErrorKind};

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
    tokio::select! {
        res = server::run(listener) => {
             if let Err(_err) = res {
               println!("failed to accept connection");               
            }
        }
        _ = shutdown => {
            println!("inside shutdown loop");
            
        }
    }
    
    // Tokio.select
// let listener = Listener.listener;

    // let db = Listner;
    // let entry_clone = db.clone();
    // db.entries
    // let entry_two = entry_clone.lock().unwrap().get(&"hello".to_string());
    // loop {
// create my data, wrap it in a mutex, then add atomic reference couting
// let my_data = Arc::new(Mutex::new(vec![1, 2, 3]));

// spawn a thread that will update the values
// a clone of our Arc will be moved into the thread
// let thread_arc = my_data.clone();


    //     let (mut socket, _) = listener.accept().await?;
    //     println!("loop run");
    //     let  db_clone = db.clone();
    //     tokio::spawn(async move{
    //         process_method(&mut socket, db_clone).await;
    //         // Ok(res)
    //     });
    // }
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
