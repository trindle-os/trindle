//! Output.

use std::{
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, Weak},
};

use tracing::{info, instrument};

#[cfg(feature = "wayland_frontend")]
use crate::wayland::output::xdg::XdgOutput;
#[cfg(feature = "backend_drm")]
use drm::control::{connector::SubPixel as DrmSubPixel, Mode as DrmMode, ModeFlags};
#[cfg(feature = "wayland_frontend")]
use std::collections::HashSet;
#[cfg(feature = "wayland_frontend")]
use wayland_server::{
    backend::WeakHandle, protocol::wl_output::WlOutput, protocol::wl_surface::WlSurface, Weak as WlWeak,
};

use crate::utils::{self, user_data::UserDataMap, Logical, Physical, Point, Raw, Size, Transform};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mode {
    pub size: Size<i32, Physical>,
    pub refresh: i32,
}

#[cfg(feature = "backend_drm")]
impl From<DrmMode> for Mode {
    #[inline]
    fn from(mode: DrmMode) -> Self {
        let clock = mode.clock() as u64;
        let htotal = mode.hsync().2 as u64;
        let vtotal = mode.vsync().2 as u64;

        let mut refresh = (clock * 1_000_000 / htotal + vtotal / 2) / vtotal;

        if mode.flags().contains(ModeFlags::INTERLACE) {
            refresh *= 2;
        }

        if mode.flags().contains(ModeFlags::DBLSCAN) {
            refresh /= 2;
        }

        if mode.vscan() > 1 {
            refresh /= mode.vscan() as u64;
        }

        let (w, h) = mode.size();

        Self {
            size: (w as i32, h as i32).into(),
            refresh: refresh as i32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subpixel {
    Unknown,
    None,
    HorizontalRgb,
    HorizontalBgr,
    VerticalRgb,
    VerticalBgr,
}

#[cfg(feature = "backend_drm")]
impl From<DrmSubPixel> for Subpixel {
    #[inline]
    fn from(mode: DrmSubPixel) -> Self {
        match mode {
            DrmSubPixel::Unknown => Self::Unknown,
            DrmSubPixel::HorizontalRgb => Self::HorizontalRgb,
            DrmSubPixel::HorizontalBgr => Self::HorizontalBgr,
            DrmSubPixel::VerticalRgb => Self::VerticalRgb,
            DrmSubPixel::VerticalBgr => Self::VerticalBgr,
            DrmSubPixel::None => Self::None,
            DrmSubPixel::NotImplemented => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalProperties {
    pub size: Size<i32, Raw>,
    pub subpixel: Subpixel,
    pub make: String,
    pub model: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Integer(i32),
    Fractional(f64),
    Custom {
        advertised_integer: i32,
        fractional: f64,
    },
}

impl Scale {
    pub fn integer_scale(&self) -> i32 {
        match self {
            Scale::Integer(scale) => *scale,
            Scale::Fractional(scale) => scale.ceil() as i32,
            Scale::Custom {
                advertised_integer, ..
            } => *advertised_integer,
        }
    }

    pub fn fractional_scale(&self) -> f64 {
        match self {
            Scale::Integer(scale) => *scale as f64,
            Scale::Fractional(scale) => *scale,
            Scale::Custom { fractional, .. } => *fractional,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Inner {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) physical: PhysicalProperties,
    pub(crate) location: Point<i32, Logical>,
    pub(crate) transform: Transform,
    pub(crate) scale: Scale,
    pub(crate) modes: Vec<Mode>,
    pub(crate) current_mode: Option<Mode>,
    pub(crate) preferred_mode: Option<Mode>,

    #[cfg(feature = "wayland_frontend")]
    pub(crate) instances: Vec<wayland_server::Weak<WlOutput>>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) handle: Option<WeakHandle>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) xdg_output: Option<XdgOutput>,
    #[cfg(feature = "wayland_frontend")]
    pub(crate) surfaces: HashSet<WlWeak<WlSurface>>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub(crate) inner: Arc<(Mutex<Inner>, UserDataMap)>,
}

#[derive(Debug, Default, Clone)]
pub struct WeakOutput {
    pub(crate) inner: Weak<(Mutex<Inner>, UserDataMap)>,
}

impl Output {
    #[instrument]
    pub fn new(name: String, physical: PhysicalProperties) -> Output {
        info!(name, "Creating new Output");

        let data = Arc::new((
            Mutex::new(Inner {
                name: name.clone(),
                description: format!("{} - {} - {}", physical.make, physical.model, name),
                #[cfg(feature = "wayland_frontend")]
                instances: Vec::new(),
                #[cfg(feature = "wayland_frontend")]
                handle: None,
                physical,
                location: (0, 0).into(),
                transform: Transform::Normal,
                scale: Scale::Integer(1),
                modes: Vec::new(),
                current_mode: None,
                preferred_mode: None,
                #[cfg(feature = "wayland_frontend")]
                xdg_output: None,
                #[cfg(feature = "wayland_frontend")]
                surfaces: HashSet::new(),
            }),
            UserDataMap::default(),
        ));

        Output { inner: data }
    }

    pub fn set_preferred(&self, mode: Mode) {
        let mut inner = self.inner.0.lock().unwrap();
        inner.preferred_mode = Some(mode);
        if inner.modes.iter().all(|&m| m != mode) {
            inner.modes.push(mode);
        }
    }

    pub fn add_mode(&self, mode: Mode) {
        let mut inner = self.inner.0.lock().unwrap();
        if inner.modes.iter().all(|&m| m != mode) {
            inner.modes.push(mode);
        }
    }

    pub fn current_mode(&self) -> Option<Mode> {
        self.inner.0.lock().unwrap().current_mode
    }

    pub fn preferred_mode(&self) -> Option<Mode> {
        self.inner.0.lock().unwrap().preferred_mode
    }

    pub fn current_transform(&self) -> Transform {
        self.inner.0.lock().unwrap().transform
    }

    pub fn current_scale(&self) -> Scale {
        self.inner.0.lock().unwrap().scale
    }

    pub fn current_location(&self) -> Point<i32, Logical> {
        self.inner.0.lock().unwrap().location
    }

    pub fn name(&self) -> String {
        self.inner.0.lock().unwrap().name.clone()
    }

    pub fn description(&self) -> String {
        self.inner.0.lock().unwrap().description.clone()
    }

    pub fn physical_properties(&self) -> PhysicalProperties {
        self.inner.0.lock().unwrap().physical.clone()
    }

    pub fn modes(&self) -> Vec<Mode> {
        self.inner.0.lock().unwrap().modes.clone()
    }

    pub fn delete_mode(&self, mode: Mode) {
        let mut inner = self.inner.0.lock().unwrap();
        inner.modes.retain(|&m| m != mode);
        if inner.current_mode == Some(mode) {
            inner.current_mode = None;
        }
        if inner.preferred_mode == Some(mode) {
            inner.preferred_mode = None;
        }
    }

    #[instrument(skip(self), fields(output = self.name()))]
    pub fn change_current_state(
        &self,
        new_mode: Option<Mode>,
        new_transform: Option<Transform>,
        new_scale: Option<Scale>,
        new_location: Option<Point<i32, Logical>>,
    ) {
        {
            let mut inner = self.inner.0.lock().unwrap();
            if let Some(mode) = new_mode {
                if inner.modes.iter().all(|&m| m != mode) {
                    inner.modes.push(mode);
                }
                inner.current_mode = new_mode;
            }
            if let Some(transform) = new_transform {
                inner.transform = transform;
            }
            if let Some(scale) = new_scale {
                inner.scale = scale;
            }
            if let Some(new_location) = new_location {
                inner.location = new_location;
            }
        }

        #[cfg(feature = "wayland_frontend")]
        self.wl_change_current_state(new_mode, new_transform.map(Into::into), new_scale, new_location)
    }

    pub fn user_data(&self) -> &UserDataMap {
        &self.inner.1
    }

    pub fn downgrade(&self) -> WeakOutput {
        WeakOutput {
            inner: Arc::downgrade(&self.inner),
        }
    }

    #[profiling::function]
    pub fn cleanup(&self) {
        #[cfg(feature = "wayland_frontend")]
        self.cleanup_surfaces();
    }
}

impl PartialEq for Output {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Output {}

impl Hash for Output {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl WeakOutput {
    #[inline]
    pub fn upgrade(&self) -> Option<Output> {
        self.inner.upgrade().map(|inner| Output { inner })
    }

    #[inline]
    pub fn is_alive(&self) -> bool {
        self.inner.strong_count() != 0
    }
}

impl PartialEq for WeakOutput {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for WeakOutput {}

impl Hash for WeakOutput {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Weak::as_ptr(&self.inner).hash(state);
    }
}

impl PartialEq<WeakOutput> for Output {
    #[inline]
    fn eq(&self, other: &WeakOutput) -> bool {
        other.upgrade().map(|o| &o == self).unwrap_or(false)
    }
}

impl PartialEq<Output> for WeakOutput {
    #[inline]
    fn eq(&self, other: &Output) -> bool {
        self.upgrade().map(|o| &o == other).unwrap_or(false)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OutputModeSource {
    Auto(Output),
    Static {
        size: Size<i32, Physical>,
        scale: utils::Scale<f64>,
        transform: Transform,
    },
}

impl From<&Output> for OutputModeSource {
    #[inline]
    fn from(output: &Output) -> Self {
        Self::Auto(output.clone())
    }
}

impl TryFrom<&OutputModeSource> for (Size<i32, Physical>, utils::Scale<f64>, Transform) {
    type Error = OutputNoMode;

    #[inline]
    fn try_from(mode: &OutputModeSource) -> Result<Self, Self::Error> {
        match mode {
            OutputModeSource::Auto(output) => {
                let guard = output.inner.0.lock().unwrap();
                Ok((
                    guard.current_mode.as_ref().ok_or(OutputNoMode)?.size,
                    guard.scale.fractional_scale().into(),
                    guard.transform,
                ))
            }
            OutputModeSource::Static {
                size,
                scale,
                transform,
            } => Ok((*size, *scale, *transform)),
        }
    }
}

impl TryFrom<OutputModeSource> for (Size<i32, Physical>, utils::Scale<f64>, Transform) {
    type Error = OutputNoMode;

    #[inline]
    fn try_from(mode: OutputModeSource) -> Result<Self, Self::Error> {
        Self::try_from(&mode)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Output has no active mode")]
pub struct OutputNoMode;
