use std::{
    io::{BufRead, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    pub fn build(stream: &mut TcpStream) -> Self {
        let mut buffer = [0; 1024];

        let size = stream.read(&mut buffer).unwrap();
        let mut request = buffer.take(size as u64).lines().map_while(Result::ok);

        let head = request.next().unwrap_or("".to_string());
        let head: Vec<&str> = head.split(' ').collect();

        let method = match head.first() {
            Some(&"GET") => Method::Get,
            Some(&"POST") => Method::Post,
            _ => panic!("invalid method"),
        };
        let path = *head.get(1).expect("no path found");
        let request: Vec<String> = request.collect();
        let mut request = request.split(|line| line.is_empty());

        Request {
            method,
            path: path.to_owned(),
            headers: request.next().unwrap().to_vec(),
            body: request
                .next()
                .unwrap()
                .to_vec()
                .first()
                .unwrap_or(&"".into())
                .to_owned(),
        }
    }
}
