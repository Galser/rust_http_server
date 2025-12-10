# Primitive HTTP server in RUST

Part of the course "Learn Rust by Building Real Applications"


# Progress milestones

- Now server is accepting connection and reading from the buffer
    Screenshot with echo 
    ![Screenshot of stream read](/screenshots/first_listner_test_with_netcat.png?raw=true "Listner and console screenshot, Netcat")
- Example of request from Chrome 
![Screenshot of stream read, Chrome request](/screenshots/first_request_from_chrome.png?raw=true "Listner screenshot of Chrome request")
- Adddress in use error : 
![Screenshot of address already in use panic in unwrap](/screenshots/addr_in_use.png "Addr already in use")
- Query debug via derives : 
![Screenshot of query with parameter via debug](/screenshots/query_debuf_with_parameters.png "Query debug screenshot")
- Query with multiple parameters debug via derives  : 
![Screenshot of query with multi parameter via debug](/screenshots/query_debug_with_multi_parameters.png "Multi parameter Query debug screenshot")
- HTTP 404 responce generated from Display trait-enabled struct
![Screenshot of 404 reponcse to CURL request](/screenshots/http_404_response.png "HTTP 404 repsonces to CURL request")


# TODO

[x] implement bare minimum
[x] make tcp stream reads and screen echo
[x] parse custom errors
[x] lifetimes
[x] derives
[x] query string as HashMap
[x] modeling HTTP response
[x] Copy/Clone
[x] Writing data to TCP stream
[x] Custom HTTP repsonses


