# httping
    Httping is like 'ping' but for http-requests.
    Provide a URL or IP address, plus the HTTP/HTTPS port number.
    Returns with a bool indicating success or failure of the ping,
    as well as how long it takes to connect send a request and retrieve
    the full response (headers + body) which is returned as an integer "rtt".

### Usage

```rust
    async fn ping_bool() {
        let wrapped_result = ping("koonts.net", "", "http", 80).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    async fn ping_bool_ip() {
        let wrapped_result = ping("", "96.30.198.61", "http", 80).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    async fn ping_bool_https() {
        let wrapped_result = ping("koonts.net", "", "https", 443).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }
    
    async fn ping_bool_ip_https() {
        let wrapped_result = ping("", "96.30.198.61", "https", 443).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    async fn ping_full() {
        let wrapped_result = ping_with_metrics("koonts.net", "", "http", 80).await;
        let result = wrapped_result.unwrap();
        let success = result.success;
        let rtt = result.rtt;
        println!("{:#?}", result);
        assert_eq!(success, true);
        assert!(rtt > 0)
    }
```


## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
