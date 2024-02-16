# httping
    Httping is like 'ping' but for http-requests.
    Provide a URL or IP address, plus the HTTP/HTTPS port number.
    Returns with a bool indicating success or failure of the ping,
    as well as how long it takes to connect send a request and retrieve
    the full response (headers + body) which is returned as an integer "rtt".


## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
