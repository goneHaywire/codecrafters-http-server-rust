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
    pub fn build(mut stream: &mut TcpStream) -> Self {
        let reader = BufReader::new(&mut stream);
        let mut request = reader
            .lines()
            .take_while(|line| !line.as_ref().unwrap().is_empty())
            .flatten();

        let head = request.next().unwrap_or("".to_string());
        let head: Vec<&str> = head.split(" ").collect();

        let method = match head.get(0) {
            Some(&"GET") => Method::GET,
            Some(&"POST") => Method::POST,
            _ => panic!("invalid method"),
        };
        let path = *head.get(1).expect("no path found");

        Request {
            method,
            path: path.to_owned(),
            headers: request.collect(),
        }
    }
}
