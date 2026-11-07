//! Helper for synchronizing rendering operations.

use std::{error::Error, fmt, os::unix::io::OwnedFd, sync::Arc};

use downcast_rs::{impl_downcast, Downcast};

#[cfg(feature = "backend_egl")]
mod egl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interrupted;

impl fmt::Display for Interrupted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Wait for Fence was interrupted")
    }
}
impl Error for Interrupted {}

pub trait Fence: std::fmt::Debug + Send + Sync + Downcast {
    fn is_signaled(&self) -> bool;

    fn wait(&self) -> Result<(), Interrupted>;

    fn is_exportable(&self) -> bool;

    fn export(&self) -> Option<OwnedFd>;
}
impl_downcast!(Fence);

#[derive(Debug, Clone)]
#[must_use = "this `SyncPoint` may contain a fence that should be awaited, failing to do so may result in unexpected rendering artifacts"]
pub struct SyncPoint {
    fence: Option<Arc<dyn Fence>>,
}

impl Default for SyncPoint {
    fn default() -> Self {
        Self::signaled()
    }
}

impl SyncPoint {
    pub fn signaled() -> Self {
        Self {
            fence: Default::default(),
        }
    }

    pub fn contains_fence(&self) -> bool {
        self.fence.is_some()
    }

    pub fn get<F: Fence + 'static>(&self) -> Option<&F> {
        self.fence.as_ref().and_then(|f| f.downcast_ref())
    }

    pub fn is_reached(&self) -> bool {
        self.fence.as_ref().map(|f| f.is_signaled()).unwrap_or(true)
    }

    #[profiling::function]
    pub fn wait(&self) -> Result<(), Interrupted> {
        if let Some(fence) = self.fence.as_ref() {
            fence.wait()
        } else {
            Ok(())
        }
    }

    pub fn is_exportable(&self) -> bool {
        self.fence.as_ref().map(|f| f.is_exportable()).unwrap_or(false)
    }

    #[profiling::function]
    pub fn export(&self) -> Option<OwnedFd> {
        self.fence.as_ref().and_then(|f| f.export())
    }
}

impl<T: Fence + 'static> From<T> for SyncPoint {
    #[inline]
    fn from(value: T) -> Self {
        SyncPoint {
            fence: Some(Arc::new(value)),
        }
    }
}
