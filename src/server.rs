use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

/// Server struc
impl Server {
    /// Returns new  instance of server with given "addr"
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

    #[deny(clippy::unused_io_amount)]
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        //  simply unwrap it, as we can fail here 
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // listener loops
        loop {
             match listener.accept() {
                Ok((mut stream, _)) =>  { 
                    println!("Listner is running");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) { 
                     Ok(_) => {
                        println!("Received a request {}", String::from_utf8_lossy(&buffer));
                        match Request::try_from(&buffer[..]) {
                            Ok(request) => { 
                                dbg!(request);
                                let response = Response::new(StatusCode::Ok, 
                                    Some("<h1>it works</h1>".to_string()));
                                write!(stream, "{}", response); 
                            },
                            Err(e) => { 
                                println!("Failed to parse a request {}", e);
//                                println!("Buffer ! {:?}", buffer);
                            },    
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
