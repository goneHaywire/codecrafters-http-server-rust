use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Request {
    pub fn build(reader: &mut BufReader<&mut TcpStream>) -> Result<Self, ()> {
        let mut method = Method::Get;
        let mut path = String::new();
        let mut headers: Vec<(String, String)> = Vec::new();

        let lines = reader.lines().enumerate();

        for (index, line) in lines {
            let line = line.unwrap_or_default();

            // end when headers are finished
            if line.is_empty() {
                break;
            }

            // process the request line
            if index == 0 {
                let mut line = line.split(' ');
                method = Method::from_str(line.next().expect("no method found"))?;
                path = line.next().expect("no path found").to_owned();
            } else {
                let header = line.trim().split_once(": ").unwrap();
                headers.push((header.0.into(), header.1.into()));
            }
        }

        Ok(Request {
            method,
            path,
            headers,
            body: None,
        })
    }

    pub fn read_body(&mut self, reader: &mut BufReader<&mut TcpStream>) -> Result<(), ()> {
        let length = self
            .headers
            .iter()
            .find(|(header, _)| header == "Content-Length")
            .map(|(_, value)| value.parse::<usize>().unwrap())
            .unwrap_or(0);

        if length == 0 {
            return Ok(());
        }

        let mut body = vec![0_u8; length];

        reader.read_exact(&mut body).unwrap_or_default();
        self.body = Some(String::from_utf8_lossy(&body).to_string());
        Ok(())
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.iter().find_map(|(header, value)| {
            if header == key {
                Some(value.as_str())
            } else {
                None
            }
        })
    }
}
