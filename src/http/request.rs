use crate::http::method::MethodError;
use crate::http::{method, request};

use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; 
use std::str;
use std::sync::Arc;


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

    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // var shadowing
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // var shadowing
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // var shadowing
        if protocol != "HTTP/1.1" { 
            return Err(ParseError::InvalidProtocol);
        }
        let method:Method = method.parse()?;  
        unimplemented!()
    }    
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r'  { return  Some((&request[..i], &request[i+1..]));}
    };
    None
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

impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self {
        Self::InvalidMethod
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
