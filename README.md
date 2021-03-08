# unix-socket-example

Example repository of a basic unix socket server and client.

This repository demonstrates stream sockets, feel free to contribute a datagram example or add a link to one in this readme. Thanks.

Run the server with:  
`cargo run -- --server`  
Open a client connection with:  
`cargo run -- --client`

## Warning! I am currently looking for a solution to the below:
### Current problems:
- [ ] One way communication only, from the client to the server.

## References:
https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
https://stackoverflow.com/q/40218416/3033813
