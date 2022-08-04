use bytes::BytesMut;
use std::{
    io::{self, Error, Read},
    thread::sleep,
    time::Duration,
};
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connection accepted {:?}", socket);

        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;

        println!("buffer {:?}", buf);
    }
    Ok(())
}
