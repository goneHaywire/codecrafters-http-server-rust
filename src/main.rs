use clap::Parser;
use encoding::{Encoding, EncodingType};
use std::{
    fs,
    io::{BufReader, Result as IOResult, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    str::FromStr,
    thread,
};

use crate::{
    request::{Method, Request},
    response::{Body, Response, StatusCode},
};

mod encoding;
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

    let mut reader = BufReader::new(&mut stream);

    let mut request = Request::build(&mut reader).expect("failed to build request");
    request.read_body(&mut reader).expect("failed to read body");

    let encodings = request
        .get_header("Accept-Encoding")
        .map(|encodings| encodings.split(", ").collect::<Vec<&str>>())
        .unwrap_or_default()
        .iter()
        .flat_map(|s: &&str| EncodingType::from_str(s))
        .collect::<Encoding>();

    let mut response = Response::new();

    response = response.set_encoding(encodings);

    if request.path == "/" {
        response = response.set_status(StatusCode::Ok).set_body(Body::Empty);
    } else if request.path.contains("/echo") {
        let text = &request.path[6..];

        response = response
            .set_status(StatusCode::Ok)
            .set_body(Body::String(text.to_owned()));
    } else if request.path.contains("/user-agent") {
        let user_agent = request.get_header("User-Agent").unwrap();

        response = response
            .set_status(StatusCode::Ok)
            .set_body(Body::String(user_agent.into()))
    } else if request.path.contains("/files") {
        match request.method {
            Method::Get => {
                if let Some(dir) = args.directory {
                    let fname = request.path.split('/').last().unwrap();
                    let path: PathBuf = [dir, fname.into()].iter().collect();
                    let metadata = fs::metadata(&path);

                    match metadata {
                        Ok(metadata) => match metadata.is_file() {
                            true => {
                                response = response
                                    .set_status(StatusCode::Ok)
                                    .set_body(Body::File(fs::read_to_string(&path).unwrap()));
                            }
                            false => {
                                response = response
                                    .set_status(StatusCode::NotFound)
                                    .set_body(Body::Empty);
                            }
                        },
                        Err(_) => {
                            response = response
                                .set_status(StatusCode::NotFound)
                                .set_body(Body::Empty);
                        }
                    }
                } else {
                    response = response
                        .set_status(StatusCode::NotFound)
                        .set_body(Body::Empty);
                }
            }
            Method::Post => {
                if let Some(dir) = args.directory {
                    let fname = request.path.split('/').last().unwrap();
                    let path: PathBuf = [dir, fname.into()].iter().collect();
                    let mut file = fs::File::create(path).unwrap();

                    file.write_all(request.body.unwrap_or("".into()).as_bytes())
                        .unwrap();

                    response = response
                        .set_status(StatusCode::Created)
                        .set_body(Body::Empty);
                } else {
                    response = response
                        .set_status(StatusCode::NotFound)
                        .set_body(Body::Empty);
                }
            }
        };
    } else {
        response = response.set_status(StatusCode::NotFound);
    };

    response.send(&mut stream)
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("address already in use");

    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle_stream(stream));
    }
}
