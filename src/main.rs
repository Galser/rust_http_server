//! Simple HTTP Server
//! main
//!
#![allow(dead_code)]

use server::Server;

pub mod server;
pub mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

