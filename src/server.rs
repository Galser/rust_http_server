use std::net::TcpListener;
use std::io::Read;
//use super::Request;
use crate::http::Request;
use std::convert::TryFrom;

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

        // let listener = match TcpListener::bind(&self.addr) {
        //     Ok(tcplistener) => tcplistener,
        //     Err(e) => panic!("Can't bind : {}", e),
        // };

        // or simply unwrap it, as we can fail here 
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // listener loops
        'main: loop {
             match listener.accept() {
                Ok((mut stream, _)) =>  { 
                    println!("Listner is running");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) { 
                     Ok(bytes) => {
                        println!("Received a request {}", String::from_utf8_lossy(&buffer));
//                        Request::try_from(&buffer as &[u8]);
                        match Request::try_from(&buffer[..]) {
                            Ok(request) => {},
                            Err(e) => println!("Failed to parse a request {}", e),
                        }
                     },
                     Err(e) => println!("failed to read from connection {}", e),

                    } // match on stream read

                },   
                Err(e) => println!("Error while accepting : {}", e),
            }
        }
    }
}
