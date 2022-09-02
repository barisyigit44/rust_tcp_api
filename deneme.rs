use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { println!("{:?}",e); }
        }
    }
}

fn parse_request(mut request_vector_u8: Vec<u8>) -> Vec<String> {

    // clean 0(null) and 34(") char
    request_vector_u8.retain(|&x| x != 0 && x!= 34);

    // convert the type of the variable from array to string
    let request_string = String::from_utf8_lossy(&request_vector_u8).to_string();

    // split from "\r\n"
    let mut request_split = request_string.split("\r\n").map(str::to_string).collect::<Vec<String>>();

    // if string contains two \r\n , vector will have an extra empty member so remove empty members.
    request_split.retain(|x| x != "");

    // return string vector
    request_split
}

fn handle_connection(mut stream: TcpStream) {

    // define buffer
    let mut buffer = [0; 256];

    // read request
    stream.read(&mut buffer).unwrap();

    // parse request
    let parsed_request = parse_request(buffer.to_vec());

    // print
    println!("{:#?}",parsed_request);

}
