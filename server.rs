use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from stream");
        if bytes_read == 0 {
            return;
        }
        stream.write(&buf[..bytes_read]).expect("Failed to write to stream");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7722").expect("Failed to bind to socket");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
