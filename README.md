# unix-socket-example

Example repository of a basic unix socket server and client.

This repository demonstrates stream sockets, feel free to contribute a datagram example or add a link to one in this readme. Thanks.

Run the server with:  
`cargo run -- --server`  
Open a client connection with:  
`cargo run -- --client`

## A handy set of conditions:
- The client's read socket closes when the server's write socket closes.
- The server's write socket closes when the server's read socket closes.
- The server's read socket closes when the the client's write socket closes.

https://stackoverflow.com/a/44076060/3033813

## References:
https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
https://stackoverflow.com/q/40218416/3033813
