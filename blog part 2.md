# Part 2

In the previous blog post, we covered single threaded client server communication. In this blog post, we will be covering multi-threaded server client communication. Every new client request will be handled in its own separate thread. We will be using `tokio::spawn` to spawn a new thread to handle the incoming request from the client.

We will also be implementing server shutdown using `ctrl + c`. It will close the server but the clients will be gracefully shutdown without panicking. For this we need to communication from server to client. Let's use `tokio::sync::broadcast` channel broadcast messages to client. But this is only possible when all the clients subscribe to the broadcast sender. This means our client will be holding at least fields `socket` and `receiver`.

Also, since multiple clients can connect simultaneously, they also read and write data simultaneously. This means, we also need to supply a copy of `Db` to each thread. As of now, we just have one instance variable of `Db`, this object needs to be shared across the threads. For this purpose, 2 things need to happen.

1. We will use `Arc<Mutex>` and wrap it around the `HashMap<String, Bytes>` in our `Db` to create shared references. We could use `Rc<RefCell>` for this purpose, but since we are talking about multi-threading `Rc` is not a great fit as it is not thread-safe.
2. Since, we are looking at many values associated with a client connection, we should create a struct `Handler` to encapsulate all the fields together.

Let's try to breakdown our code such that hitting `ctrl + c` while the server is running triggers the shutdown process for the server. In the main function of `bin/server.rs` let's capture the shutdown signal using `tokio::signal`.

```
use tokio::signal;

// in main function
let shutdown = signal::ctrl_c();
```

Whenever we hit `ctrl+ c` this `shutdown` future completes on the `.await` or `Future::poll`.

We want to run 2 branches at this point, one to handle the incoming requests from clients and the other to listen on this shutdown. So whenever `ctrl+c` is received the shutdown process starts dropping the client handle branch. This is where `tokio::select` comes into picture. It takes any number of async branches in form of futures(similar to promise in js) and runs them concurrently, waiting for value from any branch. Upon receiving value from a branch, it execuates the handler function and drops the rest of the branches.

```
tokio::select! {
    res = server::run(&mut listener) => {
            if let Err(_err) = res {
            println!("failed to accept connection");
        }
    }
    _ = shutdown => {
        println!("inside shutdown loop");
    }
}
// code runs here only when shutdown future returns value or server encounters and error.
```

In the code above, both the branches will run and wait for any one to return a value. To keep the server running, we will not return a value from `server::run` unless there is an error. If we hit `ctrl+c` at this point, then shutdown branch will run stopping our `server::run()` future.

At this point, lets create our `Handler` and `Listener` structs. The Listener struct is the server object holding `TcpListener` and broadcast channel. Handler struct is the client object holding db clone, socket, .

Let's move the main server logic to a new file `src/server.rs`. This file will hold main execution of server.

File `src/server.rs`

```
loop {
    let (mut socket, _) = listener.accept().await?;
    let mut buf = BytesMut::with_capacity(1024);
    socket.read_buf(&mut buf).await?;
    let attrs = buffer_to_array(&mut buf);
    let command = Command::get_command(&attrs[0]);
}
```
