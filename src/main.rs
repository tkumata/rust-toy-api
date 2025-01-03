mod application;
mod interface;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::UdpSocket;
use anyhow::Result;
use std::net::SocketAddr;

use presentation::handlers::convert_handler::ConvertHandler;
use presentation::handlers::generic_handler::GenericHandler;
use presentation::handlers::metrics_handler::MetricsHandler;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<()> {
    let listen_addr = "0.0.0.0:4123";
    let forward_addr = resolve_host("ntp.nict.jp:123").await?;
    println!("Forwarding to {}", forward_addr);

    // UDP Proxy Task
    let udp_task = tokio::spawn(async move {
        println!("Starting UDP proxy task...");
        if let Err(e) = run_udp_proxy(listen_addr, forward_addr).await {
            eprintln!("UDP proxy encountered an error: {:?}", e);
        }
    });

    let app = Router::new()
        // healthcheck
        .route("/healthcheck", get(GenericHandler::healthcheck))
        // Return random number
        .route("/roll/1d6", get(GenericHandler::roll_1d6))
        // Sleep
        .route("/sleep/{wait_time}", get(GenericHandler::make_sleep))
        // Get metrics.
        .route("/metrics", get(MetricsHandler::get_metrics))
        .route("/metrics/kernel", get(MetricsHandler::get_kernel))
        .route("/metrics/cpu", get(MetricsHandler::get_cpu))
        .route("/metrics/memory", get(MetricsHandler::get_memory))
        .route("/metrics/storage", get(MetricsHandler::get_storage))
        // Convert /27 to 255.255.255.224
        .route("/convert/bitv4", post(ConvertHandler::convert_bitv4))
        // Convert 55,155,250 to 379BFA
        .route("/convert/rgb", post(ConvertHandler::convert_rgb));

    // run our app with hyper, listening globally on port 4000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    // HTTP Server Task
    let http_task = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
        println!("HTTP server running on 0.0.0.0:4000");
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("HTTP server encountered an error: {:?}", e);
        }
    });

    // Wait for tasks
    tokio::try_join!(udp_task, http_task)?;

    Ok(())
}

async fn resolve_host(addr: &str) -> Result<SocketAddr> {
    let mut addrs = tokio::net::lookup_host(addr).await?;
    addrs
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to resolve address: {}", addr))
}

async fn run_udp_proxy(listen_addr: &str, forward_addr: SocketAddr) -> Result<()> {
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
