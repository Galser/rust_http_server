//! Simple HTTP Server
//! main
//!

use core::str;

fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    /// Server struc
    impl Server {
        /// Retursn new  instance of server with given "addr"
        ///
        /// # Arguments :
        ///
        /// * `addr` - Binding address
        ///
        /// # Examples :
        ///
        /// let server = Server::new("127.0.0.1:8080".to_string());
        ///
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {

    mod request {
        /// Request struc
        ///
        pub struct Request {
            path: String,
            query_string: Option<String>, // What if there is no Query .  then we have None option
            method: super::method::Method,
        }
    }

    mod method {
        /// HTTP Methods
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}