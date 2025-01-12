use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct UdpProxyService;

impl UdpProxyService {
    pub async fn run_udp123_proxy(listen_addr: &str, forward_addr: SocketAddr) -> Result<()> {
        let socket = match UdpSocket::bind(listen_addr).await {
            Ok(sock) => sock,
            Err(e) => {
                eprintln!("Failed to bind to {}: {:?}", listen_addr, e);
                return Err(anyhow::anyhow!(e));
            }
        };
        let mut buf = vec![0u8; 1024];

        println!("UDP proxy listening on {}", listen_addr);

        loop {
            // クライアントからデータを受信
            let (len, src_addr) = match socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error receiving UDP packet: {:?}", e);
                    continue;
                }
            };
            println!("Received {} bytes from {}", len, src_addr);

            // 転送先にデータを送信
            if let Err(e) = socket.send_to(&buf[..len], forward_addr).await {
                eprintln!("Error forwarding packet to {}: {:?}", forward_addr, e);
                continue;
            }
            println!("Forwarded {} bytes to {}", len, forward_addr);

            // 転送先からレスポンスを受信
            let (resp_len, _) = match socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error receiving response: {:?}", e);
                    continue;
                }
            };
            println!("Received response of {} bytes", resp_len);

            // クライアントにレスポンスを送信
            if let Err(e) = socket.send_to(&buf[..resp_len], src_addr).await {
                eprintln!("Error sending response back to {}: {:?}", src_addr, e);
            } else {
                println!("Sent response back to {}", src_addr);
            }
        }
    }
}
