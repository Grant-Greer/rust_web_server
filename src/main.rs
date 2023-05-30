use std::io::prelude::*;
use std::str;
use std::net::{TcpStream, TcpListener};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = str::from_utf8(&buffer).unwrap();
    let mut lines = request.lines();

    // Get the first line of the request (the request line)
    let request_line = match lines.next() {
        Some(line) => line,
        None => return, // If there's no data, just return
    };

    // Split the request line into components
    let components: Vec<&str> = request_line.split_whitespace().collect();
    if components.len() != 3 {
        return; // If the request line doesn't have three components, it's not valid
    }

    let method = components[0];
    let path = components[1];

    if method == "GET" && path == "/" {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
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