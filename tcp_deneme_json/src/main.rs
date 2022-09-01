use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    name: String,
    age: i16
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5656").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    
    let state = Customer {
        name: "Baris".to_string(),
        age: 23
    };

    let json = serde_json::to_string(&state).unwrap();
    
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = format!(
        "HTTP/1.0 200 OK\r\nContent-type: application/json\r\n\r\n{}",
        &json
        
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}