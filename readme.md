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


# TODO

[x] implement bare minimum
[x] make tcp stream reads and screen echo
[X] parse custom errors
[x] lifetimes
[x] derives
[x] query string as HashMap
[ ] modeling HTTP response
[ ] Copy/Clone
[ ] Writing data to TCP stream
