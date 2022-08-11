use crate::Handler;
use crate::Listener;

pub async fn run(listener: &Listener) -> std::io::Result<()> {
    let mut count = 0;
    loop {
        count += 1;
        println!("iteration {}", count);
        let socket = listener.accept().await?;
        println!("new connection accepted");
        let mut handler = Handler::new(listener, socket);

        tokio::spawn(async move {
            if let Err(_err) = process_method(&mut handler).await {
                println!("Connection Error");
            } else {
                println!("returning from process method");
            }
        });
    }
}

// async fn process_method (socket: &mut TcpStream, db: Db,) -> Result<(), std::io::Error>{
async fn process_method(handler: &mut Handler) -> Result<(), std::io::Error> {
    while !handler.shutdown.is_shutdown() {
        let result = tokio::select! {
            // biased;
            _ = handler.shutdown.listen_recv() => {
                println!("SHUTDOWN returning from recieve listen");
                return Ok(());
            },
            res = handler.connection.read_frame() => res,
        };

        let (cmd, vec) = match result {
            Some((cmd, vec)) => (cmd, vec),
            None => return Ok(()),
        };

        handler.process_query(cmd, vec).await?;
    }
    Ok(())
}
