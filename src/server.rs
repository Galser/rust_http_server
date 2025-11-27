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
