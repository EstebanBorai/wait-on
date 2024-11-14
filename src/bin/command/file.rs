use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use wait_on::resource::file::FileWaiter;
use wait_on::{WaitOptions, Waitable};

#[derive(Args, Debug)]
pub struct FileOpt {
    pub path: PathBuf,
}

impl FileOpt {
    pub async fn exec(&self, options: &WaitOptions) -> Result<()> {
        let waiter = FileWaiter::new(self.path.clone());
        waiter.wait(options).await
    }
}
