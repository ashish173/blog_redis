use crate::Handler;
use crate::Listener;

pub async fn run(listener: &Listener) -> std::io::Result<()> {
    loop {
        let socket = listener.accept().await?;
        let mut handler = Handler::new(listener, socket);

        tokio::spawn(async move {
            if let Err(_err) = process_method(&mut handler).await {
                println!("Connection Error");
            }
        });
    }
}
// async fn process_method (socket: &mut TcpStream, db: Db,) -> Result<(), std::io::Error>{
async fn process_method(handler: &mut Handler) -> Result<(), std::io::Error> {
    while !handler.shutdown.is_shutdown() {
        let (command, attrs) = tokio::select! {
            res = handler.connection.read_frame() => res?,
            _ = handler.shutdown.listen_recv() => {
                println!("returning from recieve listen");
                return Ok(());
            }
        };
        handler.process_query(command, attrs).await?;
    }
    Ok(())
}
