use anyhow::Result;
use reqwest::{Client, Method, Request, Url};

use crate::{WaitOptions, Waitable};

pub struct HttpWaiter {
    pub method: Method,
    pub url: Url,
}

impl HttpWaiter {
    pub fn new(method: Method, url: Url) -> Self {
        Self { method, url }
    }
}

impl Waitable for HttpWaiter {
    async fn wait(self, _: WaitOptions) -> Result<()> {
        let client = Client::new();
        let request = Request::new(self.method, self.url);

        loop {
            if let Some(req) = request.try_clone() {
                match client.execute(req).await {
                    Ok(res) => {
                        println!("Got {}", res.status());
                        break;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }

        Ok(())
    }
}
