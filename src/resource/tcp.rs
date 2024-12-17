use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use anyhow::Result;
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::{WaitOptions, Waitable};

/// Listens on a specific IP Address and Port using TCP protocol
pub struct TcpWaiter {
    pub addr: IpAddr,
    pub port: u16,
}

impl TcpWaiter {
    pub fn new(addr: IpAddr, port: u16) -> Self {
        Self { addr, port }
    }

    pub fn socket(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

impl Waitable for TcpWaiter {
    async fn wait(&self, _: &WaitOptions) -> Result<()> {
        let connect = || async { TcpStream::connect(self.socket()).await };

        while let Err(err) = connect().await {
            println!("Failed to connect to {}. {err}", self.socket());
            sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }
}
