//! Simple HTTP Server
//! main
//!

use core::str;
use server::Server;
use http::request::Request;

pub mod server;
pub mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

