use bytes::BytesMut;
use chrono::Local;
use reqwest::{ClientBuilder, Error};
use std::time::{Duration, SystemTime};

pub struct PingHost {
    server_domain: String,
    server_address: String,
    protocol: String,
    server_port: u32,
}

#[derive(Debug, Clone)]
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
        #[allow(unused_variables)]
        let now = Local::now().timestamp();
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut rtt = 0u64;

        let mut body = BytesMut::new();
        let sys_time = SystemTime::now();

        let mut res = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(12))
            .build()?
            .get(full_addr.clone())
            .send()
            .await?;

        while let Some(chunk) = res.chunk().await? {
            // println!("Chunk: {:?}", chunk);
            if !chunk.is_empty() {
                body.extend_from_slice(&chunk.clone())
            }
        }

        // println!("Chunk: {:?}", body);

        let post_req_sys_time = SystemTime::now();
        let rtt = post_req_sys_time
            .duration_since(sys_time)
            .expect("Could not calculate duration since")
            .as_millis();
        // println!("RTT: {}", rtt);

        Ok(if !body.is_empty() {
            PingHostResult { success: true, rtt }
        } else {
            PingHostResult {
                success: false,
                rtt,
            }
        })
    }
}
