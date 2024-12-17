use std::net::IpAddr;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use humantime::Duration;

use reqwest::{Method, Url};
use wait_on::resource::file::FileWaiter;
use wait_on::resource::http::HttpWaiter;
use wait_on::resource::tcp::TcpWaiter;
use wait_on::resource::Resource;
use wait_on::task::WaitOnTask;
use wait_on::WaitOptions;

#[derive(Debug, Parser)]
#[command(
    name = "wait-on",
    about = "Library and CLI Utility to wait on the availability of resources such as Files, HTTP Servers, Ports & Sockets",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/wait-on)",
    next_line_help = true
)]
pub enum Command {
    /// Wait on a file to be available
    File { path: PathBuf },
    /// Wait on a HTTP resource to be available
    Http { method: Method, url: Url },
    /// Wait on a TCP connection to be available
    Tcp {
        #[clap(short = 'p', long = "port")]
        port: u16,
        #[clap(short = 'i', long = "ip", default_value = "127.0.0.1")]
        addr: IpAddr,
    },
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// Timeout for waiting tasks
    #[clap(long, short = 't', default_value = "1h")]
    pub timeout: Duration,
    #[command(subcommand)]
    pub command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let options = WaitOptions {
        timeout: args.timeout.into(),
    };
    let resource: Resource = match args.command {
        Command::File { path } => Resource::File(FileWaiter::new(path)),
        Command::Http { method, url } => Resource::Http(HttpWaiter::new(method, url)),
        Command::Tcp { addr, port } => Resource::Tcp(TcpWaiter::new(addr, port)),
    };
    let wait_on_task = WaitOnTask::new(resource, options);

    wait_on_task.run().await
}
