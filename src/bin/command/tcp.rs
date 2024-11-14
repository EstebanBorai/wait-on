use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use wait_on::resource::tcp::TcpWaiter;
use wait_on::{WaitOptions, Waitable};

#[derive(Args, Debug)]
pub struct TcpOpt {
    #[clap(short = 'p', long = "port")]
    pub port: u16,
    #[clap(short = 'i', long = "ip", default_value = "127.0.0.1")]
    pub addr: IpAddr,
}

impl TcpOpt {
    pub async fn exec(&self, options: &WaitOptions) -> Result<()> {
        let waiter = TcpWaiter::new(self.addr, self.port);
        waiter.wait(options).await
    }
}
