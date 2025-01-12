use anyhow::Result;
use std::net::SocketAddr;

pub(crate) async fn resolve_host(addr: &str) -> Result<SocketAddr> {
    let mut addrs = tokio::net::lookup_host(addr).await?;
    addrs
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to resolve address: {}", addr))
}
