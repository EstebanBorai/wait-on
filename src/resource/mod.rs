//! A [`Resource`] is an object that can be waited on. [`Resource`]s hold its
//! own configuration based on the protocols used.

pub mod file;
pub mod tcp;

use anyhow::Result;

use crate::{WaitOptions, Waitable};

use self::file::FileWaiter;
use self::tcp::TcpWaiter;

pub enum Resource {
    File(FileWaiter),
    Tcp(TcpWaiter),
}

impl Waitable for Resource {
    async fn wait(self, options: WaitOptions) -> Result<()> {
        match self {
            Resource::File(file) => file.wait(options).await,
            Resource::Tcp(tcp) => tcp.wait(options).await,
        }
    }
}
