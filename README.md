# blog_redis

A redis server in server implementation. A detailed guide is covered in the [blog post](https://dev.to/ashish173/building-redis-server-in-rust-part-1-m4f).

## starting server

To start the server run `$cargo run --bin server`

## running set command

To run the client with set command run the following command
`$ cargo run --bin client set foo bar`

## running get command

To get the keys and values run the command
`$ cargo run --bin client get foo`
