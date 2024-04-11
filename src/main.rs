use std::{
    io::Result as IOResult,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    request::{Method, Request},
    response::{Response, StatusCode},
};

mod request;
mod response;

fn handle_stream(mut stream: TcpStream) -> IOResult<usize> {
    println!("accepted new connection");

    let request = Request::build(&mut stream);
    println!("{:?}", request);

    match request.method {
        Method::GET => {
            if request.path.contains("/echo") {
                let text = &request.path[6..];

                Response::build(StatusCode::Ok, text.to_owned()).send(stream)
            } else if request.path.contains("/user-agent") {
                let user_agent = request
                    .headers
                    .iter()
                    .find(|&header| header.contains("User-Agent"))
                    .map(|s| s.split(":").last().unwrap().trim())
                    .unwrap();

                Response::build(StatusCode::Ok, user_agent.into()).send(stream)
            } else if request.path == "/" {
                Response::build(StatusCode::Ok, "".into()).send(stream)
            } else {
                Response::build(StatusCode::NotFound, "".into()).send(stream)
            }
        }
        Method::POST => {
            todo!("implement post")
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || handle_stream(stream));
        }
    }
}
