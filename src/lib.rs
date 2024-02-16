use crate::ping::{PingHost, PingHostResult};

mod ping;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn ping(server_domain: &str, host_address: &str, protocol: &str, host_port: u32) -> bool {
    let host = PingHost::new(
        server_domain.to_string(),
        host_address.to_string(),
        protocol.to_string(),
        host_port,
    )
    .await;
    let result: PingHostResult = host.start().await.unwrap();
    result.success
}

pub async fn ping_with_metrics(
    server_domain: &str,
    host_address: &str,
    protocol: &str,
    host_port: u32,
) -> PingHostResult {
    let host = PingHost::new(
        server_domain.to_string(),
        host_address.to_string(),
        protocol.to_string(),
        host_port,
    )
    .await;
    let result: PingHostResult = host.start().await.unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping_bool() {
        let result = ping("koonts.net", "", "http", 80).await;
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn ping_full() {
        let result = ping_with_metrics("koonts.net", "", "http", 80).await;
        let success = result.success;
        let rtt = result.rtt;
        println!("{:#?}", result);
        assert_eq!(success, true);
        assert!(rtt > 0)
    }
}
