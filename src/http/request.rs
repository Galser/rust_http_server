        use super::method::Method;
        
        /// Request struc
        ///
        pub struct Request {
            path: String,
            query_string: Option<String>, // What if there is no Query .  then we have None option
            method: super::method::Method,
        }
