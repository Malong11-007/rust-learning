use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, FromStr, Utf8Error};

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(String::from("Invalid Method")),
        }
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

pub struct Request {
    pub path: String,
    query_string: Option<String>,
    method: HttpMethod,
    headers: Option<String>,
}

impl TryFrom<&Vec<u8>> for Request {
    type Error = ParseError;

    fn try_from(buffer: &Vec<u8>) -> Result<Self, Self::Error> {
        // echo -e "GET / HTTP/1.1\r\nHost: example.com\r\nCustom-Header: my-header-value\r\n\r\n" | nc example.com 80
        let request_string = from_utf8(buffer)?;
        let mut parted_request_data = request_string.splitn(3, " ");

        let method = match parted_request_data.next() {
            Some(method_string) => {
                HttpMethod::from_str(method_string).or(Err(ParseError::InvalidMethod))?
            }
            _ => Err(ParseError::InvalidMethod)?,
        };

        let path = match parted_request_data.next() {
            Some(path) => path,
            _ => Err(ParseError::InvalidRequest)?,
        };

        match parted_request_data.next() {
            Some(remaining_request_data) => match remaining_request_data.lines().next() {
                Some("HTTP/1.1") => {}
                _ => Err(ParseError::InvalidProtocol)?,
            },
            _ => Err(ParseError::InvalidProtocol)?,
        }

        Ok(Request {
            path: path.to_string(),
            method,
            query_string: None,
            headers: None,
        })
    }
}
