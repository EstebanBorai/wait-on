//! Wait-On library to wait on the availability of resources
//! such as Files, HTTP Servers, Ports & Sockets

pub mod resource;

use anyhow::Result;

pub type Millis = u64;

/// Options available for waiting on a [`Waitable`].
#[derive(Debug, Default)]
pub struct WaitOptions {
    /// Timeout in milliseconds for the wait operation.
    pub timeout: Option<Millis>,
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
    async fn wait(self, options: WaitOptions) -> Result<()>;
}
