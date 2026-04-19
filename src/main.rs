use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    // status http
    let status_line = "HTTP/1.1 200 OK";
    // baca file html
    let contents = fs::read_to_string("hello.html").unwrap();
    // itung panjang content
    let length = contents.len();
    
    // format respons http
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // kirim ke browser
    stream.write_all(response.as_bytes()).unwrap();
}