use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use wait_on::resource::socket::SocketWaiter;
use wait_on::{WaitOptions, Waitable};

#[derive(Args, Debug)]
pub struct SocketOpt {
    #[clap(short = 'p', long = "port")]
    pub port: u16,
    #[clap(short = 'i', long = "ip", default_value = "127.0.0.1")]
    pub addr: IpAddr,
}

impl SocketOpt {
    pub async fn exec(&self) -> Result<()> {
        let waiter = SocketWaiter::new(self.addr, self.port);
        waiter.wait(WaitOptions::default()).await
    }
}
