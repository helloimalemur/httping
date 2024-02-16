use crate::ping::PingHost;

mod ping;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn ping(server_domain: &str, host_address: &str, protocol: &str, host_port: u32) -> bool {
    let host = PingHost::new(server_domain.to_string(), host_address.to_string(), protocol.to_string(), host_port).await;
    host.start().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = ping("koonts.net", "", "http", 80).await;
        assert_eq!(result, true);
    }
}
