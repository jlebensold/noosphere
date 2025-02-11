use std::{convert::Infallible, fmt::Display, str::FromStr};

pub enum Header {
    ContentType,
    Proof,
    Author,
    Signature,
    Version,
    Unknown(String),
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Header::ContentType => "Content-Type",
            Header::Proof => "Proof",
            Header::Author => "Author",
            Header::Signature => "Signature",
            Header::Version => "Version",
            Header::Unknown(name) => name,
        };

        write!(f, "{}", value)
    }
}

impl FromStr for Header {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "content-type" => Header::ContentType,
            "proof" => Header::Proof,
            "author" => Header::Author,
            "signature" => Header::Signature,
            "version" => Header::Version,
            _ => Header::Unknown(s.to_string()),
        })
    }
}
