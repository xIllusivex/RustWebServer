use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
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

    if uri[1] == "/" {
        //opening and reading the contents of our resource
        let mut file = File::open(format!("{}/index.html", RESOURCES)).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    else if uri[1].ends_with("css") {
        let mut file = File::open(format!("{}{}", RESOURCES, uri[1])).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    else {
        let mut file = File::open(format!("{}/missing.html", RESOURCES)).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }

    // sending the resonse
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
