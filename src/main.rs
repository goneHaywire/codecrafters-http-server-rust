use std::{io::Write, net::TcpListener};

use crate::request::{Method, Request};

mod request;
mod response;

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
                        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                    } else {
                        stream
                            .write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes())
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
