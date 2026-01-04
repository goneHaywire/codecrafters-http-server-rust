use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum EncodingType {
    Gzip,
}

impl Display for EncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gzip => "gzip",
            }
        )
    }
}

impl FromStr for EncodingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Self::Gzip),
            _ => Err(()),
        }
    }
}

pub(crate) type Encoding = Vec<EncodingType>;
