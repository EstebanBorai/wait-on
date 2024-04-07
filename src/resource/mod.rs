//! A [`Resource`] is an object that can be waited on. [`Resource`]s hold its
//! own configuration based on the protocols used.

pub mod file;
pub mod socket;

use anyhow::Result;

use crate::{WaitOptions, Waitable};

use self::file::FileWaiter;
use self::socket::SocketWaiter;

pub enum Resource {
    File(FileWaiter),
    Socket(SocketWaiter),
}

impl Waitable for Resource {
    async fn wait(self, options: WaitOptions) -> Result<()> {
        match self {
            Resource::File(file) => file.wait(options).await,
            Resource::Socket(socket) => socket.wait(options).await,
        }
    }
}
