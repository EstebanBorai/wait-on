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
            _ = self.watch() => Ok(()),
            _ = self.deadline() => bail!("Timeout reached"),
        }
    }

    async fn watch(&self) -> Result<()> {
        let resource = self.resource.clone();
        let options = self.options.clone();

        tokio::spawn(async move { resource.wait(&options).await }).await??;

        Ok(())
    }

    async fn deadline(&self) -> Result<()> {
        sleep(self.options.timeout).await;
        bail!("Timeout reached");
    }
}
