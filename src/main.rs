use std::alloc::{alloc, dealloc, Layout};
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
        }
        Err(e) => {
            println!("Failed to read from socket: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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

    drop(listener);
}