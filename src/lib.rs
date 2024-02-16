use crate::ping::{PingHost, PingHostResult};
use reqwest::Error;

mod ping;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn ping(
    server_domain: &str,
    host_address: &str,
    protocol: &str,
    host_port: u32,
) -> Result<bool, Error> {
    let host = PingHost::new(
        server_domain.to_string(),
        host_address.to_string(),
        protocol.to_string(),
        host_port,
    )
    .await;
    let result: PingHostResult = host.start().await?;
    Ok(result.success)
}

pub async fn ping_with_metrics(
    server_domain: &str,
    host_address: &str,
    protocol: &str,
    host_port: u32,
) -> Result<PingHostResult, Error> {
    let host = PingHost::new(
        server_domain.to_string(),
        host_address.to_string(),
        protocol.to_string(),
        host_port,
    )
    .await;
    let result: PingHostResult = host.start().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping_bool() {
        let wrapped_result = ping("koonts.net", "", "http", 80).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn ping_bool_ip() {
        let wrapped_result = ping("", "96.30.198.61", "http", 80).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn ping_bool_https() {
        let wrapped_result = ping("koonts.net", "", "https", 443).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn ping_bool_ip_https() {
        let wrapped_result = ping("", "96.30.198.61", "https", 443).await;
        let result = wrapped_result.unwrap();
        println!("{:#?}", result);
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn ping_full() {
        let wrapped_result = ping_with_metrics("koonts.net", "", "http", 80).await;
        let result = wrapped_result.unwrap();
        let success = result.success;
        let rtt = result.rtt;
        println!("{:#?}", result);
        assert_eq!(success, true);
        assert!(rtt > 0)
    }
}
