//! Linux DMABUF protocol.

mod dispatch;

use std::{
    collections::HashMap,
    ops::Sub,
    os::unix::io::AsFd,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use indexmap::{IndexMap, IndexSet};
use rustix::fs::{seek, SeekFrom};
use wayland_protocols::wp::linux_dmabuf::zv1::server::{
    zwp_linux_buffer_params_v1::{self, ZwpLinuxBufferParamsV1},
    zwp_linux_dmabuf_feedback_v1, zwp_linux_dmabuf_v1,
};
use wayland_server::{
    backend::{GlobalId, InvalidId},
    protocol::{
        wl_buffer::{self, WlBuffer},
        wl_surface::WlSurface,
    },
    Client, Dispatch, DisplayHandle, GlobalDispatch, Resource, WEnum,
};

#[cfg(feature = "backend_drm")]
use crate::backend::drm::DrmNode;
use crate::{
    backend::allocator::{
        dmabuf::{Dmabuf, DmabufFlags, Plane},
        Format, Fourcc, Modifier,
    },
    utils::{ids::id_gen, SealedFile, UnmanagedResource},
};

use super::{buffer::BufferHandler, compositor};

#[derive(Debug, Clone, PartialEq)]
struct DmabufFeedbackTranche {
    target_device: libc::dev_t,
    flags: zwp_linux_dmabuf_feedback_v1::TrancheFlags,
    indices: IndexSet<usize>,
}

#[derive(Debug)]
struct DmabufFeedbackFormatTable {
    formats: IndexSet<Format>,
    file: SealedFile,
}

#[derive(Debug)]
struct DmabufFeedbackInner {
    main_device: libc::dev_t,
    format_table: DmabufFeedbackFormatTable,
    tranches: Vec<DmabufFeedbackTranche>,
}

impl PartialEq for DmabufFeedbackInner {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.main_device == other.main_device
            && self.format_table.formats == other.format_table.formats
            && self.tranches == other.tranches
    }
}

#[derive(Debug, Clone)]
pub struct DmabufFeedbackBuilder {
    main_device: libc::dev_t,
    main_tranche: DmabufFeedbackTranche,
    formats: IndexSet<Format>,
    preferred_tranches: Vec<DmabufFeedbackTranche>,
}

#[derive(Copy, Clone)]
struct DmabufFeedbackFormat {
    format: u32,
    _reserved: u32,
    modifier: u64,
}

impl DmabufFeedbackFormat {
    fn to_ne_bytes(self) -> [u8; 16] {
        let format: [u8; 4] = self.format.to_ne_bytes();
        let reserved: [u8; 4] = self._reserved.to_ne_bytes();
        let modifier: [u8; 8] = self.modifier.to_ne_bytes();

        [
            format[0],
            format[1],
            format[2],
            format[3],
            reserved[0],
            reserved[1],
            reserved[2],
            reserved[3],
            modifier[0],
            modifier[1],
            modifier[2],
            modifier[3],
            modifier[4],
            modifier[5],
            modifier[6],
            modifier[7],
        ]
    }
}

impl From<Format> for DmabufFeedbackFormat {
    #[inline]
    fn from(format: Format) -> Self {
        DmabufFeedbackFormat {
            format: format.code as u32,
            _reserved: 0,
            modifier: format.modifier.into(),
        }
    }
}

impl DmabufFeedbackBuilder {
    pub fn new(main_device: libc::dev_t, formats: impl IntoIterator<Item = Format>) -> Self {
        let feedback_formats: IndexSet<Format> = formats.into_iter().collect();
        let format_indices: IndexSet<usize> = (0..feedback_formats.len()).collect();
        let main_tranche = DmabufFeedbackTranche {
            flags: zwp_linux_dmabuf_feedback_v1::TrancheFlags::empty(),
            indices: format_indices,
            target_device: main_device,
        };

        Self {
            main_device,
            formats: feedback_formats,
            main_tranche,
            preferred_tranches: Vec::new(),
        }
    }

    pub fn add_preference_tranche(
        mut self,
        target_device: libc::dev_t,
        flags: Option<zwp_linux_dmabuf_feedback_v1::TrancheFlags>,
        formats: impl IntoIterator<Item = Format>,
    ) -> Self {
        let flags = flags.unwrap_or(zwp_linux_dmabuf_feedback_v1::TrancheFlags::empty());

        let mut tranche = DmabufFeedbackTranche {
            target_device,
            flags,
            indices: Default::default(),
        };

        for format in formats {
            let (format_index, added) = self.formats.insert_full(format);

            let duplicate_format = !added
                && self.preferred_tranches.iter().any(|tranche| {
                    tranche.target_device == target_device
                        && tranche.flags == flags
                        && tranche.indices.contains(&format_index)
                });

            if duplicate_format {
                continue;
            }

            tranche.indices.insert(format_index);
        }

        if !tranche.indices.is_empty() {
            self.preferred_tranches.push(tranche);
        }

        self
    }

    pub fn build(mut self) -> Result<DmabufFeedback, std::io::Error> {
        let formats = self
            .formats
            .iter()
            .copied()
            .map(DmabufFeedbackFormat::from)
            .flat_map(DmabufFeedbackFormat::to_ne_bytes)
            .collect::<Vec<_>>();

        let name = c"smithay-dmabuffeedback-format-table";
        let format_table_file = SealedFile::with_data(name, &formats)?;

        for duplicate_main_tranche in self.preferred_tranches.iter().filter(|tranche| {
            tranche.target_device == self.main_tranche.target_device
                && tranche.flags == self.main_tranche.flags
        }) {
            self.main_tranche.indices = self.main_tranche.indices.sub(&duplicate_main_tranche.indices);
        }

        if !self.main_tranche.indices.is_empty() {
            self.preferred_tranches.push(self.main_tranche);
        }

        Ok(DmabufFeedback(Arc::new(DmabufFeedbackInner {
            main_device: self.main_device,
            format_table: DmabufFeedbackFormatTable {
                file: format_table_file,
                formats: self.formats,
            },
            tranches: self.preferred_tranches,
        })))
    }
}

#[derive(Debug, Clone)]
pub struct DmabufFeedback(Arc<DmabufFeedbackInner>);

impl PartialEq for DmabufFeedback {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.0, &other.0) {
            return true;
        }

        self.0 == other.0
    }
}

impl DmabufFeedback {
    pub fn send(&self, feedback: &zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1) {
        feedback.main_device(self.0.main_device.to_ne_bytes().to_vec());
        feedback.format_table(
            self.0.format_table.file.as_fd(),
            self.0.format_table.file.size() as u32,
        );

        for tranche in self.0.tranches.iter() {
            feedback.tranche_target_device(tranche.target_device.to_ne_bytes().to_vec());
            feedback.tranche_flags(tranche.flags);
            feedback.tranche_formats(
                tranche
                    .indices
                    .iter()
                    .flat_map(|i| (*i as u16).to_ne_bytes())
                    .collect::<Vec<_>>(),
            );
            feedback.tranche_done();
        }

        feedback.done();
    }

    fn main_formats(&self) -> Vec<Format> {
        self.0
            .tranches
            .iter()
            .filter(|tranche| tranche.target_device == self.0.main_device && tranche.flags.is_empty())
            .map(|tranche| tranche.indices.clone())
            .reduce(|mut acc, item| {
                acc.extend(item);
                acc
            })
            .unwrap_or_default()
            .into_iter()
            .map(|index| self.0.format_table.formats[index])
            .collect()
    }
}

#[derive(Debug)]
struct SurfaceDmabufFeedbackStateInner {
    feedback: DmabufFeedback,
    known_instances: Vec<wayland_server::Weak<zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1>>,
}

#[derive(Debug, Clone, Default)]
pub struct SurfaceDmabufFeedbackState {
    inner: Arc<Mutex<Option<SurfaceDmabufFeedbackStateInner>>>,
}

impl SurfaceDmabufFeedbackState {
    pub fn from_states(states: &compositor::SurfaceData) -> Option<&Self> {
        states.data_map.get::<SurfaceDmabufFeedbackState>()
    }

    pub fn set_feedback(&self, feedback: &DmabufFeedback) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(inner) = guard.as_mut() {
            if &inner.feedback == feedback {
                return;
            }

            for instance in inner.known_instances.iter().filter_map(|i| i.upgrade().ok()) {
                feedback.send(&instance);
            }

            inner.feedback = feedback.clone();
        }
    }

    fn add_instance<F>(
        &self,
        instance: &zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1,
        feedback_factory: F,
    ) -> DmabufFeedback
    where
        F: FnOnce() -> DmabufFeedback,
    {
        let mut guard = self.inner.lock().unwrap();
        if let Some(inner) = guard.as_mut() {
            inner.known_instances.push(instance.downgrade());
            inner.feedback.clone()
        } else {
            let feedback = feedback_factory();
            let inner = SurfaceDmabufFeedbackStateInner {
                feedback: feedback.clone(),
                known_instances: vec![instance.downgrade()],
            };
            *guard = Some(inner);
            feedback
        }
    }

    fn remove_instance(&self, instance: &zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1) {
        let mut guard = self.inner.lock().unwrap();

        let reset = if let Some(inner) = guard.as_mut() {
            inner.known_instances.retain(|i| i != instance);
            inner.known_instances.is_empty()
        } else {
            false
        };
        if reset {
            *guard = None;
        }
    }
}

#[derive(Debug)]
struct DmabufGlobalState {
    id: GlobalId,

    default_feedback: Option<Arc<Mutex<DmabufFeedback>>>,
    known_default_feedbacks:
        Arc<Mutex<Vec<wayland_server::Weak<zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1>>>>,
}

#[derive(Debug)]
pub struct DmabufState {
    globals: HashMap<usize, DmabufGlobalState>,
}

impl DmabufState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> DmabufState {
        DmabufState {
            globals: HashMap::new(),
        }
    }

    pub fn create_global<D>(
        &mut self,
        display: &DisplayHandle,
        formats: impl IntoIterator<Item = Format>,
    ) -> DmabufGlobal
    where
        D: GlobalDispatch<zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, DmabufGlobalData>
            + BufferHandler
            + DmabufHandler
            + 'static,
    {
        self.create_global_with_filter::<D, _>(display, formats, |_| true)
    }

    pub fn create_global_with_filter<D, F>(
        &mut self,
        display: &DisplayHandle,
        formats: impl IntoIterator<Item = Format>,
        filter: F,
    ) -> DmabufGlobal
    where
        D: GlobalDispatch<zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, DmabufGlobalData>
            + BufferHandler
            + DmabufHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let formats = formats.into_iter().collect::<Vec<_>>();
        self.create_global_with_filter_and_optional_default_feedback::<D, _>(
            display,
            Some(formats),
            None,
            filter,
        )
    }

    pub fn create_global_with_default_feedback<D>(
        &mut self,
        display: &DisplayHandle,
        default_feedback: &DmabufFeedback,
    ) -> DmabufGlobal
    where
        D: GlobalDispatch<zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, DmabufGlobalData>
            + BufferHandler
            + DmabufHandler
            + 'static,
    {
        self.create_global_with_filter_and_default_feedback::<D, _>(display, default_feedback, |_| true)
    }

    pub fn create_global_with_filter_and_default_feedback<D, F>(
        &mut self,
        display: &DisplayHandle,
        default_feedback: &DmabufFeedback,
        filter: F,
    ) -> DmabufGlobal
    where
        D: GlobalDispatch<zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, DmabufGlobalData>
            + BufferHandler
            + DmabufHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        self.create_global_with_filter_and_optional_default_feedback::<D, _>(
            display,
            None,
            Some(default_feedback),
            filter,
        )
    }

    fn create_global_with_filter_and_optional_default_feedback<D, F>(
        &mut self,
        display: &DisplayHandle,
        formats: Option<Vec<Format>>,
        default_feedback: Option<&DmabufFeedback>,
        filter: F,
    ) -> DmabufGlobal
    where
        D: GlobalDispatch<zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, DmabufGlobalData>
            + BufferHandler
            + DmabufHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let id = global_id::next();

        let formats = formats
            .or_else(|| default_feedback.map(|f| f.main_formats()))
            .unwrap()
            .into_iter()
            .fold(
                IndexMap::<Fourcc, IndexSet<Modifier>>::new(),
                |mut formats, format| {
                    if let Some(modifiers) = formats.get_mut(&format.code) {
                        modifiers.insert(format.modifier);
                    } else {
                        formats.insert(format.code, IndexSet::from_iter(std::iter::once(format.modifier)));
                    }
                    formats
                },
            );

        let formats = Arc::new(formats);
        let version = if default_feedback.is_some() { 5 } else { 3 };

        let known_default_feedbacks = Arc::new(Mutex::new(Vec::new()));
        let default_feedback = default_feedback.map(|f| Arc::new(Mutex::new(f.clone())));

        let data = DmabufGlobalData {
            filter: Box::new(filter),
            formats,
            default_feedback: default_feedback.clone(),
            known_default_feedbacks: known_default_feedbacks.clone(),
            id,
        };

        let global = display.create_global::<D, zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1, _>(version, data);
        self.globals.insert(
            id,
            DmabufGlobalState {
                id: global,
                default_feedback,
                known_default_feedbacks,
            },
        );

        DmabufGlobal { id }
    }

    pub fn set_default_feedback(&self, global: &DmabufGlobal, default_feedback: &DmabufFeedback) {
        let Some(global) = self.globals.get(&global.id) else {
            return;
        };

        let Some(mut current_feedback) = global.default_feedback.as_ref().map(|f| f.lock().unwrap()) else {
            return;
        };

        if &*current_feedback == default_feedback {
            return;
        }

        let known_default_feedbacks = global.known_default_feedbacks.lock().unwrap();
        for feedback in known_default_feedbacks.iter().filter_map(|f| f.upgrade().ok()) {
            default_feedback.send(&feedback);
        }

        *current_feedback = default_feedback.clone();
    }

    pub fn disable_global<D: 'static>(&mut self, display: &DisplayHandle, global: &DmabufGlobal) {
        if let Some(global_state) = self.globals.get(&global.id) {
            display.disable_global::<D>(global_state.id.clone());
        }
    }

    pub fn destroy_global<D: 'static>(&mut self, display: &DisplayHandle, global: DmabufGlobal) {
        if global_id::remove(global.id) {
            if let Some(global_state) = self.globals.remove(&global.id) {
                display.remove_global::<D>(global_state.id);
            }
        }
    }
}

#[allow(missing_debug_implementations)]
pub struct DmabufGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
    formats: Arc<IndexMap<Fourcc, IndexSet<Modifier>>>,
    default_feedback: Option<Arc<Mutex<DmabufFeedback>>>,
    known_default_feedbacks:
        Arc<Mutex<Vec<wayland_server::Weak<zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1>>>>,
    id: usize,
}

#[derive(Debug)]
pub struct DmabufData {
    formats: Arc<IndexMap<Fourcc, IndexSet<Modifier>>>,
    id: usize,

    default_feedback: Option<Arc<Mutex<DmabufFeedback>>>,
    known_default_feedbacks:
        Arc<Mutex<Vec<wayland_server::Weak<zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1>>>>,
}

#[derive(Debug)]
pub struct DmabufFeedbackData {
    known_default_feedbacks:
        Arc<Mutex<Vec<wayland_server::Weak<zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1>>>>,
    surface: Option<wayland_server::Weak<wayland_server::protocol::wl_surface::WlSurface>>,
}

#[derive(Debug)]
pub struct DmabufParamsData {
    id: usize,

    used: AtomicBool,

    formats: Arc<IndexMap<Fourcc, IndexSet<Modifier>>>,

    modifier: Mutex<Option<Modifier>>,
    planes: Mutex<Vec<Plane>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct DmabufGlobal {
    id: usize,
}

#[must_use = "This object must be used to notify the client whether dmabuf import succeeded"]
#[derive(Debug)]
pub struct ImportNotifier {
    inner: ZwpLinuxBufferParamsV1,
    display: DisplayHandle,
    dmabuf: Dmabuf,
    import: Import,
    drop_ignore: bool,
}

#[derive(Debug)]
enum Import {
    Falliable,

    Infallible(WlBuffer),
}

impl ImportNotifier {
    pub fn client(&self) -> Option<Client> {
        self.inner.client()
    }

    pub fn successful<D>(mut self) -> Result<WlBuffer, InvalidId>
    where
        D: Dispatch<zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1, DmabufParamsData>
            + Dispatch<wl_buffer::WlBuffer, Dmabuf>
            + BufferHandler
            + DmabufHandler
            + 'static,
    {
        let client = self.inner.client();

        let result = match self.import {
            Import::Falliable => {
                if let Some(client) = client {
                    match client.create_resource::<wl_buffer::WlBuffer, Dmabuf, D>(
                        &self.display,
                        1,
                        self.dmabuf.clone(),
                    ) {
                        Ok(buffer) => {
                            self.inner.created(&buffer);
                            Ok(buffer)
                        }

                        Err(err) => {
                            tracing::error!("failed to create protocol object for \"create\" request");
                            Err(err)
                        }
                    }
                } else {
                    tracing::error!("client was dead while creating wl_buffer resource");
                    self.inner.post_error(
                        zwp_linux_buffer_params_v1::Error::InvalidWlBuffer,
                        "create_immed failed and produced an invalid wl_buffer",
                    );
                    Err(InvalidId)
                }
            }
            Import::Infallible(ref buffer) => Ok(buffer.clone()),
        };

        self.drop_ignore = true;
        result
    }

    pub fn incomplete(mut self) {
        self.inner.post_error(
            zwp_linux_buffer_params_v1::Error::Incomplete,
            "missing or too many planes to create a buffer",
        );
        self.drop_ignore = true;
    }

    pub fn invalid_dimensions(mut self) {
        self.inner.post_error(
            zwp_linux_buffer_params_v1::Error::InvalidDimensions,
            "width or height of dmabuf is invalid",
        );
        self.drop_ignore = true;
    }

    pub fn invalid_format(mut self) {
        self.inner.post_error(
            zwp_linux_buffer_params_v1::Error::InvalidFormat,
            "format and plane combination are not valid",
        );
        self.drop_ignore = true;
    }

    pub fn failed(mut self) {
        if matches!(self.import, Import::Falliable) {
            self.inner.failed();
        } else {
            self.inner.post_error(
                zwp_linux_buffer_params_v1::Error::InvalidWlBuffer,
                "create_immed failed and produced an invalid wl_buffer",
            );
        }
        self.drop_ignore = true;
    }

    fn new(params: ZwpLinuxBufferParamsV1, display: DisplayHandle, dmabuf: Dmabuf, import: Import) -> Self {
        Self {
            inner: params,
            display,
            dmabuf,
            import,
            drop_ignore: false,
        }
    }
}

impl Drop for ImportNotifier {
    fn drop(&mut self) {
        if !self.drop_ignore {
            tracing::warn!(
                "Compositor bug: Server ignored ImportNotifier for {:?}",
                self.inner
            );
        }
    }
}

pub trait DmabufHandler: BufferHandler {
    fn dmabuf_state(&mut self) -> &mut DmabufState;

    fn dmabuf_imported(&mut self, global: &DmabufGlobal, dmabuf: Dmabuf, notifier: ImportNotifier);

    fn new_surface_feedback(
        &mut self,
        _surface: &WlSurface,
        _global: &DmabufGlobal,
    ) -> Option<DmabufFeedback> {
        None
    }
}

pub fn get_dmabuf(buffer: &wl_buffer::WlBuffer) -> Result<&Dmabuf, UnmanagedResource> {
    buffer.data::<Dmabuf>().ok_or(UnmanagedResource)
}

#[macro_export]
macro_rules! delegate_dmabuf {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __ZwpLinuxDmabufV1 =
            $crate::reexports::wayland_protocols::wp::linux_dmabuf::zv1::server::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;
        type __ZwpLinuxBufferParamsV1 =
            $crate::reexports::wayland_protocols::wp::linux_dmabuf::zv1::server::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1;
        type __ZwpLinuxDmabufFeedbackv1 =
            $crate::reexports::wayland_protocols::wp::linux_dmabuf::zv1::server::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1;

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwpLinuxDmabufV1: $crate::wayland::dmabuf::DmabufGlobalData
        ] => $crate::wayland::dmabuf::DmabufState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwpLinuxDmabufV1: $crate::wayland::dmabuf::DmabufData
        ] => $crate::wayland::dmabuf::DmabufState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwpLinuxBufferParamsV1: $crate::wayland::dmabuf::DmabufParamsData
        ] => $crate::wayland::dmabuf::DmabufState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_buffer::WlBuffer: $crate::backend::allocator::dmabuf::Dmabuf
        ] => $crate::wayland::dmabuf::DmabufState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __ZwpLinuxDmabufFeedbackv1: $crate::wayland::dmabuf::DmabufFeedbackData
        ] => $crate::wayland::dmabuf::DmabufState);

    };
}

impl DmabufParamsData {
    fn ensure_unused(&self, params: &ZwpLinuxBufferParamsV1) -> bool {
        if !self.used.load(Ordering::Relaxed) {
            return true;
        }

        params.post_error(
            zwp_linux_buffer_params_v1::Error::AlreadyUsed,
            "This buffer_params has already been used to create a buffer.",
        );

        false
    }

    fn create_dmabuf(
        &self,
        params: &ZwpLinuxBufferParamsV1,
        width: i32,
        height: i32,
        format: u32,
        flags: WEnum<zwp_linux_buffer_params_v1::Flags>,
        _node: Option<libc::dev_t>,
    ) -> Option<Dmabuf> {
        if !self.ensure_unused(params) {
            return None;
        }

        self.used.store(true, Ordering::Relaxed);

        let format = match Fourcc::try_from(format) {
            Ok(format) => format,
            Err(_) => {
                params.post_error(
                    zwp_linux_buffer_params_v1::Error::InvalidFormat,
                    format!("Format {:x} is not supported", format),
                );

                return None;
            }
        };

        if !self.formats.contains_key(&format) {
            params.post_error(
                zwp_linux_buffer_params_v1::Error::InvalidFormat,
                format!("Format {:?}/{:x} is not supported.", format, format as u32),
            );
            return None;
        }

        if width < 1 {
            params.post_error(
                zwp_linux_buffer_params_v1::Error::InvalidDimensions,
                "invalid width",
            );
        }

        if height < 1 {
            params.post_error(
                zwp_linux_buffer_params_v1::Error::InvalidDimensions,
                "invalid height",
            );
        }

        let mut planes = self.planes.lock().unwrap();

        for plane in &*planes {
            let end = match plane
                .stride
                .checked_mul(height as u32)
                .and_then(|o| o.checked_add(plane.offset))
            {
                Some(e) => e,

                None => {
                    params.post_error(
                        zwp_linux_buffer_params_v1::Error::OutOfBounds,
                        format!("Size overflow for plane {}.", plane.plane_idx),
                    );

                    return None;
                }
            };

            if let Ok(size) = seek(&plane.fd, SeekFrom::End(0)) {
                let _ = seek(&plane.fd, SeekFrom::Start(0));

                if plane.offset as u64 > size {
                    params.post_error(
                        zwp_linux_buffer_params_v1::Error::OutOfBounds,
                        format!("Invalid offset {} for plane {}.", plane.offset, plane.plane_idx),
                    );

                    return None;
                }

                if (plane.offset + plane.stride) as u64 > size {
                    params.post_error(
                        zwp_linux_buffer_params_v1::Error::OutOfBounds,
                        format!("Invalid stride {} for plane {}.", plane.stride, plane.plane_idx),
                    );

                    return None;
                }

                if plane.plane_idx == 0 && end as u64 > size {
                    params.post_error(
                        zwp_linux_buffer_params_v1::Error::OutOfBounds,
                        format!(
                            "Invalid stride ({}) or height ({}) for plane {}.",
                            plane.stride, height, plane.plane_idx
                        ),
                    );

                    return None;
                }
            }
        }

        let modifier = self.modifier.lock().unwrap().unwrap_or(Modifier::Invalid);
        let mut buf = Dmabuf::builder(
            (width, height),
            format,
            modifier,
            DmabufFlags::from_bits_truncate(flags.into()),
        );

        for (i, plane) in planes.drain(..).enumerate() {
            let offset = plane.offset;
            let stride = plane.stride;
            buf.add_plane(plane.into(), i as u32, offset, stride);
        }

        #[cfg(feature = "backend_drm")]
        if let Some(node) = _node.and_then(|node| DrmNode::from_dev_id(node).ok()) {
            buf.set_node(node);
        }

        let dmabuf = match buf.build() {
            Some(buf) => buf,

            None => {
                params.post_error(
                    zwp_linux_buffer_params_v1::Error::Incomplete as u32,
                    "Provided buffer is incomplete, it has zero planes",
                );
                return None;
            }
        };

        Some(dmabuf)
    }
}

id_gen!(global_id);
