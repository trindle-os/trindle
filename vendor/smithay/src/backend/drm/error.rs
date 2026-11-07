use crate::backend::SwapBuffersError;
use drm::control::{connector, crtc, plane, Mode, RawResourceHandle};
use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};

#[derive(Debug, thiserror::Error)]
#[error("DRM access error: {errmsg} on device `{dev:?}` ({source:})")]
pub struct AccessError {
    pub(crate) errmsg: &'static str,
    pub(crate) dev: Option<PathBuf>,
    #[source]
    pub source: io::Error,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to aquire DRM master")]
    DrmMasterFailed,
    #[error(transparent)]
    Access(#[from] AccessError),
    #[error("Unable to determine device id of drm device")]
    UnableToGetDeviceId(#[source] rustix::io::Errno),
    #[error("Device is currently paused, operation rejected")]
    DeviceInactive,
    #[error("Mode `{0:?}` is not compatible with all given connectors")]
    ModeNotSuitable(Mode),
    #[error("Crtc `{0:?}` is already in use by another surface")]
    CrtcAlreadyInUse(crtc::Handle),
    #[error("Surface of crtc `{0:?}` would have no connectors, which is not accepted")]
    SurfaceWithoutConnectors(crtc::Handle),
    #[error("Plane `{1:?}` is not compatible for use with crtc `{0:?}`")]
    PlaneNotCompatible(crtc::Handle, plane::Handle),
    #[error("No Plane has been specified which is not supported by the underlying implementation")]
    NoPlane,
    #[error("Non-Primary Planes (provided was `{0:?}`) are not available for use with legacy devices")]
    NonPrimaryPlane(plane::Handle),
    #[error("Clearing the framebuffer on plane `{0:?}` is not supported")]
    NoFramebuffer(plane::Handle),
    #[error("The configuration is not supported on plane `{0:?}`")]
    UnsupportedPlaneConfiguration(plane::Handle),
    #[error("No encoder found for the given connector '{connector:?}' on crtc `{crtc:?}`")]
    NoSuitableEncoder {
        connector: connector::Handle,
        crtc: crtc::Handle,
    },
    #[error("The connector ({0:?}) is unknown")]
    UnknownConnector(connector::Handle),
    #[error("The crtc ({0:?}) is unknown")]
    UnknownCrtc(crtc::Handle),
    #[error("The plane ({0:?}) is unknown")]
    UnknownPlane(plane::Handle),
    #[error("The DrmDevice is missing a required property '{name}' for handle ({handle:?})")]
    UnknownProperty {
        handle: RawResourceHandle,
        name: &'static str,
    },
    #[error("Atomic Test failed for new properties on crtc ({0:?})")]
    TestFailed(crtc::Handle),
}

impl From<Error> for SwapBuffersError {
    #[inline]
    fn from(err: Error) -> SwapBuffersError {
        match err {
            x @ Error::DeviceInactive => SwapBuffersError::TemporaryFailure(Box::new(x)),
            Error::Access(AccessError {
                errmsg, dev, source, ..
            }) if matches!(
                source.kind(),
                ErrorKind::PermissionDenied | ErrorKind::WouldBlock | ErrorKind::Interrupted
            ) || rustix::io::Errno::from_io_error(&source) == Some(rustix::io::Errno::BUSY) =>
            {
                SwapBuffersError::TemporaryFailure(Box::new(Error::Access(AccessError {
                    errmsg,
                    dev,
                    source,
                })))
            }
            x => SwapBuffersError::ContextLost(Box::new(x)),
        }
    }
}
