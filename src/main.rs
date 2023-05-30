use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str;

struct Route {
    path: &'static str,
    method: &'static str,
    handler: fn() -> &'static str, // A function pointer to the handler function
}

fn handle_root() -> &'static str {
    "You requested the root path!"
}

fn handle_hello() -> &'static str {
    "Hello, world!"
}

fn handle_client(mut stream: TcpStream, routes: &[Route]) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = str::from_utf8(&buffer).unwrap();
    let mut lines = request.lines();

    let request_line = match lines.next() {
        Some(line) => line,
        None => return,
    };

    let components: Vec<&str> = request_line.split_whitespace().collect();
    if components.len() != 3 {
        return;
    }

    let method = components[0];
    let path = components[1];

    let mut status_line = "";
    let mut body = "";
    for route in routes {
        if method == route.method && path == route.path {
            status_line = "HTTP/1.1 200 OK";
            body = (route.handler)();
            break;
        }
    }

    if status_line.is_empty() {
        status_line = "HTTP/1.1 404 NOT FOUND";
        body = "Not Found";
    }

    let response = format!(
        "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        body.len(),
        body
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    let routes = [
        Route {
            path: "/",
            method: "GET",
            handler: handle_root,
        },
        Route {
            path: "/hello",
            method: "GET",
            handler: handle_hello,
        },
    ];

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream, &routes);
    }
}
