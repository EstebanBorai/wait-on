use anyhow::{bail, Result};
use tokio::select;
use tokio::time::sleep;

use crate::resource::Resource;
use crate::{WaitOptions, Waitable};

pub struct WaitOnTask {
    resource: Resource,
    options: WaitOptions,
}

impl WaitOnTask {
    pub fn new(resource: Resource, options: WaitOptions) -> Self {
        Self { resource, options }
    }

    pub async fn run(self) -> Result<()> {
        select! {
            _ = self.resource.wait(&self.options) => Ok(()),
            _ = self.deadline() => bail!("Deadline reached"),
        }
    }

    async fn deadline(&self) -> Result<()> {
        sleep(self.options.timeout).await;
        bail!("Timeout reached");
    }
}
