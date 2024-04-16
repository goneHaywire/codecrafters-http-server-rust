use clap::Parser;
use std::{
    fs,
    io::{Result as IOResult, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use crate::{
    request::{Method, Request},
    response::{Body, Response, StatusCode},
};

mod request;
mod response;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    directory: Option<String>,
}

fn handle_stream(mut stream: TcpStream) -> IOResult<usize> {
    println!("accepted new connection");

    let args = Cli::parse();

    let request = Request::build(&mut stream);

    match request.method {
        Method::Get => {
            if request.path.contains("/echo") {
                let text = &request.path[6..];

                Response::new(StatusCode::Ok, Body::String(text.to_owned())).send(stream)
            } else if request.path.contains("/user-agent") {
                let user_agent = request
                    .headers
                    .iter()
                    .find(|&header| header.contains("User-Agent"))
                    .map(|s| s.split(':').last().unwrap().trim())
                    .unwrap();

                Response::new(StatusCode::Ok, Body::String(user_agent.into())).send(stream)
            } else if request.path.contains("/files") {
                (if let Some(dir) = args.directory {
                    let fname = request.path.split('/').last().unwrap();
                    let path: PathBuf = [dir, fname.into()].iter().collect();
                    let metadata = fs::metadata(&path);

                    match metadata {
                        Ok(metadata) => match metadata.is_file() {
                            true => {
                                let file = fs::read_to_string(&path).unwrap();
                                Response::new(StatusCode::Ok, Body::File(file))
                            }
                            false => Response::new(StatusCode::NotFound, Body::Empty),
                        },
                        Err(_) => Response::new(StatusCode::NotFound, Body::Empty),
                    }
                } else {
                    Response::new(StatusCode::NotFound, Body::Empty)
                })
                .send(stream)
            } else if request.path == "/" {
                Response::new(StatusCode::Ok, Body::Empty).send(stream)
            } else {
                Response::new(StatusCode::NotFound, Body::Empty).send(stream)
            }
        }
        Method::Post => {
            if request.path.contains("/files") {
                (if let Some(dir) = args.directory {
                    let fname = request.path.split('/').last().unwrap();
                    let path: PathBuf = [dir, fname.into()].iter().collect();
                    let mut file = fs::File::create(path).unwrap();

                    file.write_all(request.body.as_bytes()).unwrap();

                    Response::new(StatusCode::Created, Body::Empty)
                } else {
                    Response::new(StatusCode::NotFound, Body::Empty)
                })
                .send(stream)
            } else {
                Response::new(StatusCode::Ok, Body::Empty).send(stream)
            }
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle_stream(stream));
    }
}
