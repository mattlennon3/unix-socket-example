use clap::{App, Arg};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::Path};
use serde::{Serialize, Deserialize};

static SOCKET_NAME: &str = "/tmp/rs-server-unix.socket";

#[derive(serde::Serialize, Deserialize, Debug)]
struct Example {
    prop: u32
}

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

        let listener = UnixListener::bind(socket_path).unwrap();

        thread::spawn(move || socket_server(listener));

        // Wait forever for new connections
        loop {}
    } else if matches.is_present("client") {
        let stream = UnixStream::connect(Path::new(SOCKET_NAME)).unwrap();

        thread::spawn(move || {

            let client_data = Example {
                prop: 300
            };

            // Very important to end with a newline so the server doesn't get stuck waiting for one via read_line
            let serialized = format!("{}\n", serde_json::to_string(&client_data).unwrap());

            let mut writer = BufWriter::new(&stream);
            writer
                .write_all(serialized.as_bytes())
                .expect("client could not write");
            writer.flush().expect("client could not flush");

            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader
                .read_line(&mut response)
                .expect("client could not read");

            println!("Client received: {:?}", response);
        });

        // Forced to keep the main process running long enough for the server to read before closing the pipe on the client side
        sleep(Duration::from_secs(2));
    }
}

fn socket_server(listener: UnixListener) {
    // https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Reading socket");
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Server error: {}", err);
                break;
            }
        }
    }
}

fn handle_client(stream: UnixStream) {
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).expect("could not read");
    println!("Server received: {:?}", response);
    
    let deserialized: Example = serde_json::from_str(&response).unwrap();

    println!("N: {}", deserialized.prop);

    let mut writer = BufWriter::new(&stream);
    writer
        .write_all("Hi client, I am the server\n".as_bytes())
        .expect("server could not write");
    writer.flush().expect("server could not flush");
}

pub fn reset_socket(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => {
            println!("Removing previous socket: {}", path.to_str().unwrap_or(""))
        }
        Err(_err) => (),
    }
}
