use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!(
        "request: {}",
        String::from_utf8_lossy(&buffer[..])
    );


    let response = b"HTTP/1.1 200 OK\r\n\
    Content-type: text/plain; charset=utf-8\r\n\
    Connection: close\r\n\r\n\
    This is a snippet of a plain text file.\
    ";
    stream.write(response).unwrap();
    stream.flush().unwrap();
}

