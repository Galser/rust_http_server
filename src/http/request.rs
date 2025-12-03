use super::method::Method;
use std::convert::TryFrom;

/// Request struc
///
pub struct Request {
    path: String,
    query_string: Option<String>, // What if there is no Query .  then we have None option
    method: super::method::Method,
}

impl Request {
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        
    // }
    
}


impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }    
}