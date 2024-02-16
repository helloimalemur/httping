use reqwest::ClientBuilder;
use std::process;
use std::time::Duration;

pub struct PingHost {
    server_domain: String,
    server_address: String,
    protocol: String,
    server_port: u32,
}

impl PingHost {
    pub async fn new(server_domain: String, server_address: String, protocol: String, server_port: u32) -> PingHost {
        PingHost {
            server_domain,
            server_address,
            protocol,
            server_port,
        }
    }

    pub async fn start(&self) -> bool {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(7))
            .build();

        let mut full_addr = String::new();

        if self.server_domain.is_empty() && !self.server_address.is_empty() {
            full_addr = format!(
                "{}://{}:{}",
                self.protocol.clone(),
                self.server_address.clone(),
                self.server_port.clone()
            );
        } else if self.server_address.is_empty() && !self.server_domain.is_empty() {
            full_addr = format!(
                "{}://{}:{}",
                self.protocol.clone(),
                self.server_domain.clone(),
                self.server_port.clone()
            );
        }


        println!("{}", full_addr);

        let response = match client {
            Ok(r) => r.get(full_addr).send().await,
            Err(_e) => process::exit(2),
        };

        return if response.is_ok() { true } else { false };
    }
}
