use reqwest::{ClientBuilder, Error};
use std::process;
use std::time::{Duration, SystemTime};
use chrono::Local;
use bytes::BytesMut;

pub struct PingHost {
    server_domain: String,
    server_address: String,
    protocol: String,
    server_port: u32,
}

#[derive(Debug)]
pub struct PingHostResult {
    pub success: bool,
    pub rtt: u128,
}

impl PingHost {
    pub async fn new(
        server_domain: String,
        server_address: String,
        protocol: String,
        server_port: u32,
    ) -> PingHost {
        PingHost {
            server_domain,
            server_address,
            protocol,
            server_port,
        }
    }

    pub async fn start(&self) -> Result<PingHostResult, Error> {
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

        // println!("{}", full_addr);

        let now = Local::now().timestamp();
        let mut rtt = 0u64;

        let mut body = BytesMut::new();
        let sys_time = SystemTime::now();

        let mut res = reqwest::get(full_addr).await.unwrap();

        while let Some(chunk) = res.chunk().await.unwrap() {
            // println!("Chunk: {:?}", chunk);
            if &chunk.len() > &0 {
                body.extend_from_slice(&chunk.clone())
            }
        }

        // println!("Chunk: {:?}", body);

        let post_req_sys_time = SystemTime::now();
        let rtt = post_req_sys_time.duration_since(sys_time).unwrap().as_millis();
        // println!("RTT: {}", rtt);



        return Ok(if !body.is_empty() {
            PingHostResult { success: true, rtt }
        } else {
            PingHostResult { success: false, rtt }
        });
    }
}
