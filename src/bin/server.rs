use std::io::ErrorKind;

use blog_redis::{helper::buffer_to_array, Command, Db};
use bytes::BytesMut;
use tokio::{
    io::AsyncReadExt,
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let mut db = Db::new();
    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;
        let attrs = buffer_to_array(&mut buf);
        let command = Command::get_command(&attrs[0]);
        process_query(command, attrs, &mut socket, &mut db).await?;
    }
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db,
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            let result = db.read(&attrs);
            match result {
                Ok(result) => {
                    socket.write_all(&result).await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }

            Ok(())
        }
        Command::Set => {
            let resp = db.write(&attrs);
            match resp {
                Ok(result) => {
                    socket.write_all(&result.as_bytes()).await?;
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }

            Ok(())
        }
        Command::Invalid => {
            socket.write_all(b"invalid command").await?;
            Err(std::io::Error::from(ErrorKind::InvalidData))
        }
    }
}
