mod command;

use anyhow::Result;
use clap::Parser;

use self::command::file::FileOpt;
use self::command::socket::SocketOpt;

#[derive(Debug, Parser)]
#[command(
    name = "wait-on",
    about = "Library and CLI Utility to wait on the availability of resources such as Files, HTTP Servers, Ports & Sockets",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/wait-on)",
    next_line_help = true
)]
pub enum Command {
    /// Wait on a file to be available
    File(FileOpt),
    /// Wait on a socket to be available
    Socket(SocketOpt),
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::File(opt) => opt.exec().await,
        Command::Socket(opt) => opt.exec().await,
    }
}