extern crate rust_web_server;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::time::Duration;
use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream); 
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // the root resources folder.
    const RESOURCES: &str = "./src/resources";
    // reading from the tcpstream and writing the contents onto the buffer.
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let uri = String::from_utf8_lossy(&buffer);
    let uri: Vec<&str> = uri.split("\r\n").next().unwrap().split(" ").collect();
    let mut contents = String::new();
    let mut status = "200 OK";

    // checking the uri for a valid route.
    let filename = if uri[1] == "/" {
        "/index.html"
    } else if uri[1] == "/admin" {
        "/admin.html"
    }  else if uri[1].ends_with("css") {
        uri[1]
    } else {
        status = "404 NOT FOUND";
        "/missing.html"
    };
    // reading the contents of the file and passing it to the contents variable
    let mut file = File::open(format!("{}{}", RESOURCES, filename)).unwrap();
    file.read_to_string(&mut contents).unwrap();

    // sending the response
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
