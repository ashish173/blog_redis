use bytes::BytesMut;
use clap::{Parser, Subcommand};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    match args.command {
        Command::Set { key, value } => {
            stream.write_all(b"set").await?;
            stream.write_all(b" ").await?;

            stream.write_all(&key.as_bytes()).await?;
            stream.write_all(b" ").await?;

            stream.write_all(&value.as_bytes()).await?;
            let mut buf = BytesMut::with_capacity(1024);
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
                    // failed to convert bytes into string slice
                    println!("error: {}", err);
                }
            }
        }
        Command::Get { key } => {
            stream.write_all(b"get").await?;
            stream.write_all(b" ").await?;

            stream.write_all(&key.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "" {
                        println!("no such key found");
                    } else {
                        println!("key: {} => value: {}", key, resp);
                    }
                }
                Err(_err) => {
                    println!("in errr");
                }
            }
            return Ok(());
        }
    }

    Ok(())
}
