use std::{
    fmt::Display,
    io::{self, Read, Write},
    net::TcpStream,
};

use flate2::{Compression, read::GzEncoder};

use crate::encoding::{Encoding, EncodingType};

#[derive(Debug, Clone, Copy, Default)]
pub enum StatusCode {
    #[default]
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
                Self::Created => "Created",
            }
        )
    }
}

#[derive(Debug, Default)]
pub enum Body {
    String(Vec<u8>),
    File(Vec<u8>),
    #[default]
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

    fn content(&self) -> &[u8] {
        match self {
            Self::Empty => b"",
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

#[derive(Debug, Default)]
pub struct Response {
    status: StatusCode,
    body: Body,
    encoding: Encoding,
}

impl Response {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn send(self, stream: &mut TcpStream) -> io::Result<usize> {
        let encoding_line = if !self.encoding.is_empty() {
            &format!(
                "Content-Encoding: {}\r\n",
                &self
                    .encoding
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            ""
        };

        let _ = stream.write(
            format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n",
                self.status as usize,
                self.status,
                self.body.content_type(),
                self.body.len(),
                encoding_line,
            )
            .as_bytes(),
        )?;
        stream.write(self.body.content())
    }

    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_body(mut self, body: Body) -> Self {
        if self.encoding.is_empty() {
            self.body = body;
            self
        } else {
            match self.encoding.first().unwrap() {
                EncodingType::Gzip => {
                    let mut encoded = vec![];
                    self.body = match body {
                        Body::String(contents) => {
                            let mut e = GzEncoder::new(&contents[..], Compression::default());
                            let _ = e.read_to_end(&mut encoded);

                            Body::String(encoded)
                        }
                        Body::File(contents) => {
                            let mut e = GzEncoder::new(&contents[..], Default::default());
                            let _ = e.read_to_end(&mut encoded);

                            Body::File(encoded)
                        }
                        Body::Empty => Body::Empty,
                    };
                }
            }
            self
        }
    }

    pub fn set_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }
}
