use std::{
    fmt::Display,
    io::{self, Write},
    net::TcpStream,
};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    Created = 201,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Ok => "OK",
                Self::NotFound => "Not Found",
                Self::Created => "CREATED",
            }
        )
    }
}

#[derive(Debug)]
pub enum Body {
    String(String),
    File(String),
    Empty,
}

impl Body {
    fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::String(content) => content.len(),
            Self::File(content) => content.len(),
        }
    }

    fn content(&self) -> &str {
        match self {
            Self::Empty => "",
            Self::File(content) => content,
            Self::String(content) => content,
        }
    }

    fn content_type(&self) -> &str {
        match self {
            Self::Empty => "text/plain",
            Self::File(_) => "application/octet-stream",
            Self::String(_) => "text/plain",
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: Body,
}

impl Response {
    pub fn new(status: StatusCode, body: Body) -> Self {
        Response { status, body }
    }

    pub fn send(self, stream: &mut TcpStream) -> io::Result<usize> {
        stream.write(self.to_string().as_bytes())
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status as usize,
            self.status,
            self.body.content_type(),
            self.body.len(),
            self.body.content()
        )
    }
}
