use std::{io::Write, net::TcpListener};

use crate::{
    request::{Method, Request},
    response::StatusCode,
};

mod request;
mod response;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("accepted new connection");

            let request = Request::build(&mut stream);
            println!("{:?}", request);

            match request.method {
                Method::GET => {
                    if request.path.contains("/echo") {
                        let text = &request.path[6..];
                        stream
                            .write(
                                format!(
                                    "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    StatusCode::Ok as usize,
                                    StatusCode::Ok,
                                    text.len(),
                                    text
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    } else if request.path == "/" {
                        stream
                            .write(
                                format!(
                                    "HTTP/1.1 {} {}\r\n\r\n",
                                    StatusCode::Ok as usize,
                                    StatusCode::Ok
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    } else {
                        stream
                            .write(
                                format!(
                                    "HTTP/1.1 {} {}\r\n\r\n",
                                    StatusCode::NotFound as usize,
                                    StatusCode::NotFound
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    }
                }
                Method::POST => {
                    todo!("implement post")
                }
            }
        }
    }
}
