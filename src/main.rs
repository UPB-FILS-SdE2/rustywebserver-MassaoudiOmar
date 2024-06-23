use std::{
    collections::HashMap, env, fs, io::prelude::*, net::{TcpListener, TcpStream}
};

#[derive(Debug)]
struct HttpRequest {
    reqtype: String,
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn parse_http_request(req: String) -> Result<HttpRequest, String> {
    // Convert the raw request to a string

    
    // Split the request into lines
    let mut lines = req.split("\r\n");
    
    // Parse the request line (first line)
    let request_line = lines.next().ok_or("Missing request line")?;
    let mut parts = request_line.split_whitespace();
    let reqtype = parts.next().ok_or("Missing reqtype")?.to_string();
    let path = parts.next().ok_or("Missing path")?.to_string();

    // Skip the HTTP version
    parts.next().ok_or("Missing HTTP version")?;
    
    // Parse the headers
    let mut headers = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            break; // End of headers
        }
        let mut header_parts = line.splitn(2, ": ");
        let key = header_parts.next().ok_or("Malformed header")?.to_string();
        let value = header_parts.next().ok_or("Malformed header")?.to_string();
        headers.insert(key, value);
    }
    
    // The remaining lines are the body
    let body = lines.collect::<Vec<&str>>().join("\r\n");
    let body = if body.is_empty() { None } else { Some(body) };
    
    // Create the HttpRequest struct
    Ok(HttpRequest {
        reqtype,
        path,
        headers,
        body,
    })
}
fn main() {
    
    let args: Vec<String> = env::args().collect();

    let arg_root = args[2].clone();

  
    
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Server listening on 0.0.0.0:8000");
    println!("Root folder: {}" , arg_root);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, arg_root.clone());
    }
}


fn handle_connection(mut stream: TcpStream, root_folder: String) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let string_req = String::from_utf8_lossy(&buffer[..]).to_string();

    let parse_req = parse_http_request(string_req).unwrap();

    let req_path = parse_req.path.clone();
    let mut path = root_folder.clone();
    path.push_str(req_path.as_str());

    

    let contents = fs::read(path.clone()).unwrap();



    let response = b"HTTP/1.1 200 OK\r\n\
    Content-type: text/plain; charset=utf-8\r\n\
    Connection: close\r\n\r\n";

    stream.write(response).unwrap();
    stream.write(&contents).unwrap();
    
    stream.flush().unwrap();
}

