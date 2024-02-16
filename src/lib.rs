use crate::ping::PingHost;

mod ping;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn ping(host_address: String, host_port: u32) -> bool {
    let host = PingHost::new(host_address, host_port).await;
    host.start().await
}

// #[cfg(tokio::test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     async fn it_works() {
//         let result = ping("8.8.8.8".to_string(), 80).await;
//         assert_eq!(result, 4);
//     }
// }
