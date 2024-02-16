use reqwest::ClientBuilder;
use std::process;
use std::time::Duration;

pub struct PingHost {
    server_domain: String,
    server_address: String,
    server_port: u32,
}

impl PingHost {
    pub async fn new(server_domain: String, server_address: String, server_port: u32) -> PingHost {
        PingHost {
            server_domain,
            server_address,
            server_port,
        }
    }

    pub async fn start(&self) -> bool {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(7))
            .build();

        let full_addr = format!(
            "{}:{}",
            self.server_address.clone(),
            self.server_port.clone()
        );

        let response = match client {
            Ok(r) => r.get(full_addr).send().await,
            Err(_e) => process::exit(2),
        };

        return if response.is_ok() { true } else { false };
    }
}
