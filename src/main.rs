use core::panic;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
enum Method {
    GET,
    POST,
}

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
    headers: Vec<String>,
}

impl Request {
    fn new(mut stream: &mut TcpStream) -> Self {
        let reader = BufReader::new(&mut stream);
        let request: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap_or("".into()))
            .take_while(|line| !line.is_empty())
            .collect();

        let head: Vec<String> = request
            .first()
            .unwrap_or(&("".to_string()))
            .split(" ")
            .into_iter()
            .map(|str| str.to_string())
            .collect();

        let method = match head.get(0).map(|x| &x[..]) {
            Some("GET") => Method::GET,
            Some("POST") => Method::POST,
            _ => panic!("invalid method"),
        };
        let path = head.get(1).expect("no path found");

        Request {
            method,
            path: path.to_owned(),
            headers: request.into_iter().skip(1).collect(),
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("accepted new connection");

            let request = Request::new(&mut stream);
            println!("{:?}", request);

            match request.method {
                Method::GET => {
                    if request.path == "/" {
                        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    } else {
                        stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").unwrap();
                    }
                }
                Method::POST => {
                    todo!("implement post")
                }
            }
        }
    }
}
