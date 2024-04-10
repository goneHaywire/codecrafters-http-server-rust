use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<String>,
}

impl Request {
    pub fn new(mut stream: &mut TcpStream) -> Self {
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
