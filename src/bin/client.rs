use std::fmt::Error;
use std::io::ErrorKind;

use bytes::BytesMut;
use clap::{Parser, Subcommand};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    for i in 0..100 {
        println!("in clinet loop ==={}",i);
        client_process(args.clone(), i).await?;  
    }
    // client_process(args).await?;
    Ok(())
}

async fn client_process(args: Cli, i: i32) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    match args.command {
        Command::Set { key, value } => {
            stream.write_all(b"set").await?;
            stream.write_all(b" ").await?;

            stream.write_all(i.to_string().as_bytes() ).await?;
            stream.write_all(b" ").await?;

            stream.write_all(&value.as_bytes()).await?;
            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            println!("data read in buffer");
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "r Ok" {
                        println!("updated key");
                    } else if resp == "Ok" {
                        println!("key set");
                    }
                    Ok(())
                }
                Err(err) => {
                    // failed to convert bytes into string slice
                    println!("error: {}", err);
                    Err(std::io::Error::new(ErrorKind::InvalidData, "invalid data"))
                }
            }
            // Ok(())
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
}
