use anyhow::Result;
use clap::Args;

use reqwest::{Method, Url};
use wait_on::resource::http::HttpWaiter;
use wait_on::{WaitOptions, Waitable};

#[derive(Args, Debug)]
pub struct HttpOpt {
    pub method: Method,
    pub url: Url,
}

impl HttpOpt {
    pub async fn exec(&self, options: &WaitOptions) -> Result<()> {
        let waiter = HttpWaiter::new(self.method.clone(), self.url.clone());
        waiter.wait(options).await
    }
}
