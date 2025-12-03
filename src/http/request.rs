use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; 

/// Request struc
///
pub struct Request {
    path: String,
    query_string: Option<String>, // What if there is no Query .  then we have None option
    method: super::method::Method,
}

impl TryFrom<&[u8]> for Request {
/// Raw request example 
/// GET /test HTTP/1.1
/// Host: 127.0.0.1:8080
/// Connection: keep-alive
/// Cache-Control: max-age=0
/// sec-ch-ua: "Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99"
/// sec-ch-ua-mobile: ?0
/// sec-ch-ua-platform: "macOS"
/// Upgrade-Insecure-Requests: 1
/// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36
/// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
/// Sec-Fetch-Site: none
/// Sec-Fetch-Mode: navigate
/// Sec-Fetch-User: ?1
/// Sec-Fetch-Dest: document
/// Accept-Encoding: gzip, deflate, br, zstd
/// Accept-Language: en-US,en;q=0.9,ru;q=0.8,uk;q=0.7,nl;q=0.6

    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }    
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol, // not HTTP 1.1 
    InvalidMethod,
}


impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}!", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}!", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Inbalid Method",
        }
    }
}

impl Error for ParseError {}
