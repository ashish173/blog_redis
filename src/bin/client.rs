use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;

    let mut buf = BytesMut::with_capacity(1024);
    stream.write_all(b"set foo bar").await?;

    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            if resp == "r Ok" {
                println!("updated key");
            } else if resp == "Ok" {
                println!("key set");
            }
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }

    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    stream.write_all(b"get foo").await?;

    let mut buf = BytesMut::with_capacity(1024);
    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            if resp == "" {
                println!("no such key found");
            } else {
                println!("value: {}", resp);
            }
        }
        Err(_err) => {
            println!("in errr");
        }
    }
    Ok(())
}
