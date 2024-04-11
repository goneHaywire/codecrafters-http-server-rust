use std::fmt::Display;

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
    pub status: usize,
    pub body: String,
}

impl Response {
    // pub fn build(status: usize, content: impl Buf) -> Self {}
}
