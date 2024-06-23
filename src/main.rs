use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
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


    let response = "Root folder: /Users/bender/software/sde2/rustywebserver-alexandruradovici/rustywebserver-tests/public
Server listening on 0.0.0.0:8000
GET 127.0.0.1 /plain.txt -> 200 (OK)";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

