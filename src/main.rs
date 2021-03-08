
use clap::{App, Arg};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread::sleep;
use std::io::prelude::*;

use std::time::Duration;
use std::{fs, path::Path};
use std::thread;

static SOCKET_NAME: &str = "/tmp/rs-server-unix.socket";

fn main() {

    let matches = App::new("unix-socket-example")
        .version("0.1.0")
        .author("Matt Lennon")
        .arg(Arg::with_name("server").long("server").help("Start daemon"))
        .arg(
            Arg::with_name("client")
                .long("client")
                .help("Send some text to be read by the server and returned"),
        )
        .get_matches();

    if matches.is_present("server") {
        let socket_path = Path::new(SOCKET_NAME);
        reset_socket(&socket_path);

        let listener =
            UnixListener::bind(socket_path).unwrap();
        
        thread::spawn(move || socket_server(listener));

        // Wait forever for new connections
        loop {}
        
    } else if matches.is_present("client") {
        let mut stream = UnixStream::connect(Path::new(SOCKET_NAME)).unwrap();
        stream.set_write_timeout(Some(Duration::from_millis(1000))).unwrap();

        thread::spawn(move || {
            stream.write_all(b"I am the client").unwrap();
            stream.flush().unwrap();
        });

        // Forced to keep the process open long enough for the server to read before closing the pipe
        sleep(Duration::from_secs(2));
    }
}


fn socket_server(listener: UnixListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
                println!("Reading socket");
                thread::spawn(|| handle_client(stream));
                println!("Done reading!");
            }
            Err(err) => {
                println!("Server error: {}", err);
                break;
            }
        }
    }
}

fn handle_client(mut stream: UnixStream) {
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("Server: Got something: {:?}", response);
}

pub fn reset_socket(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => { println!("Removing previous socket") }
        Err(_err) => ()
    }
}