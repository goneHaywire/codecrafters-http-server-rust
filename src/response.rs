use std::{
    fmt::Display,
    io::{self, Write},
    net::TcpStream,
};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Ok => "OK",
                Self::NotFound => "NOT FOUND",
            }
        )
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

impl Response {
    pub fn build(status: StatusCode, body: String) -> Self {
        Response { status, body }
    }

    pub fn send(self, mut stream: TcpStream) -> io::Result<usize> {
        stream.write(self.to_string().as_bytes())
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            self.status as usize,
            self.status,
            self.body.len(),
            self.body
        )
    }
}
