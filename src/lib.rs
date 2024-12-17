//! Wait-On library to wait on the availability of resources
//! such as Files, HTTP Servers, Ports & Sockets

pub mod resource;
pub mod task;

use std::time::Duration;

use anyhow::Result;

const SECONDS_IN_HOUR: u64 = 3600;

/// Options available for waiting on a [`Waitable`].
#[derive(Clone, Debug)]
pub struct WaitOptions {
    /// Timeout in milliseconds for the wait operation.
    pub timeout: Duration,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(SECONDS_IN_HOUR),
        }
    }
}

/// A [`Waitable`] is an resource that can be waited on.
///
/// Every [`Resource`] must implement this trait.
///
/// # Should not be implemented by the user
///
/// This trait should not be implemented by the user, instead, it should be
/// implemented by the [`Resource`] enum variants in the `lib` scope.
#[allow(async_fn_in_trait)]
pub trait Waitable {
    async fn wait(&self, options: &WaitOptions) -> Result<()>;
}
