
use log::{info, warn};
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

struct Route {
    method: &'static str,
    path: &'static str,
    handler: fn(&str) -> String,
}

fn handle_homepage(_: &str) -> String {
    fs::read_to_string("index.html").unwrap()
}

fn handle_echo(request_body: &str) -> String {
    if request_body.is_empty() {
        return String::from("No message to echo");
    }

    // Parse the JSON body
    let parsed_json: serde_json::Value = match serde_json::from_str(request_body) {
        Ok(json) => json,
        Err(_) => return String::from("Invalid JSON"),
    };

    // Get the "message" field and echo it back
    match parsed_json.get("message") {
        Some(message) => format!("You said: {}", message),
        None => String::from("No message found"),
    }
}

fn handle_connection(mut stream: TcpStream, routes: &[Route]) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    info!("Received a request: {}", String::from_utf8_lossy(&buffer[..]));

    let request = String::from_utf8_lossy(&buffer[..]);

    let mut lines = request.lines();

    let request_line = match lines.next() {
        Some(line) => line,
        None => return,
    };

    let components: Vec<&str> = request_line.split_whitespace().collect();

    if components.len() < 3 {
        let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\nMalformed request line";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        warn!("Malformed request line: {}", request_line);
        return;
    }

    let method = components[0];
    let path = components[1];
    let version = components[2];

    if method != "GET" {
        let response = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\nMethod not allowed";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        warn!("Method not allowed: {}", method);
        return;
    }

    if version != "HTTP/1.1" {
        let response = "HTTP/1.1 505 HTTP VERSION NOT SUPPORTED\r\n\r\nHTTP Version not supported";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        warn!("Unsupported HTTP version: {}", version);
        return;
    }

    let mut parts = request.splitn(2, "\r\n\r\n");
    let headers = parts.next().unwrap_or_default();
    let body = parts.next().unwrap_or_default();

    let (status_line, body) = match routes.iter().find(|route| route.path == path && route.method == method) {
            Some(route) => ("HTTP/1.1 200 OK\r\n\r\n", (route.handler)(body)),
            None => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", String::from("Route not found")),
        };
    
        let response = format!("{}{}", status_line, body);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    
        info!("Response sent");
    }
    
    fn main() {
        env_logger::init();
    
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        info!("Listening on {}", "127.0.0.1:7878");
    
        let routes = [
            Route { method: "GET", path: "/", handler: handle_homepage },
            Route { method: "POST", path: "/echo", handler: handle_echo },
        ];
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream, &routes);
        }
    }
    
