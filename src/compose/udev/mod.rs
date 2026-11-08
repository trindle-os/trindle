// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

mod backend;
mod capture_service;
mod cards;
mod connectors;
mod console_switch;
mod dmabuf;
mod frame;
mod gamma;
mod lease;
mod output_config;
mod paint;
mod recovery;
mod resting;
pub mod run;

use std::{collections::hash_map::HashMap, time::Instant};

use smithay::{
    backend::{
        allocator::gbm::GbmAllocator,
        drm::{
            compositor::FrameFlags,
            exporter::gbm::GbmFramebufferExporter,
            output::{DrmOutput, DrmOutputManager},
            CreateDrmNodeError, DrmDeviceFd, DrmError, DrmNode,
        },
        egl,
        renderer::{
            element::memory::MemoryRenderBuffer,
            gles::GlesRenderer,
            multigpu::{gbm::GbmGlesBackend, GpuManager, MultiRenderer},
        },
        session::libseat::{self, LibSeatSession},
    },
    desktop::utils::OutputPresentationFeedback,
    output::Output,
    reexports::{
        calloop::RegistrationToken,
        drm::control::{connector, crtc, Mode as DrmMode},
        wayland_server::{backend::GlobalId, DisplayHandle},
    },
    utils::{Monotonic, Time},
    wayland::{
        dmabuf::{DmabufGlobal, DmabufState},
        drm_lease::{DrmLease, DrmLeaseState},
        drm_syncobj::DrmSyncobjState,
    },
};

use crate::{
    drawing::PointerElement,
    drm_extras::drm_scanner::DrmScanner,
    protocols::screens::ScreenMode,
    state::{AnvilState, Backend, SurfaceDmabufFeedback},
};

type UdevRenderer<'a> = MultiRenderer<
    'a,
    'a,
    GbmGlesBackend<GlesRenderer, DrmDeviceFd>,
    GbmGlesBackend<GlesRenderer, DrmDeviceFd>,
>;

#[derive(Debug, PartialEq)]
struct UdevOutputId {
    device_id: DrmNode,
    crtc: crtc::Handle,
}

pub struct UdevData {
    pub session: LibSeatSession,
    dh: DisplayHandle,
    dmabuf_state: Option<(DmabufState, DmabufGlobal)>,
    syncobj_state: Option<DrmSyncobjState>,
    primary_gpu: DrmNode,
    gpus: GpuManager<GbmGlesBackend<GlesRenderer, DrmDeviceFd>>,
    backends: HashMap<DrmNode, BackendData>,

    retired_leases: HashMap<DrmNode, DrmLeaseState>,
    pointer_images: Vec<(xcursor::parser::Image, u32, MemoryRenderBuffer)>,
    pointer_element: PointerElement,
    pointer_image: crate::cursor::Cursor,
    keyboards: Vec<smithay::reexports::input::Device>,

    pointers: Vec<smithay::reexports::input::Device>,

    pub newcomers: crate::newcomers::Newcomers,
}

type OutputManager = DrmOutputManager<
    GbmAllocator<DrmDeviceFd>,
    GbmFramebufferExporter<DrmDeviceFd>,
    Option<OutputPresentationFeedback>,
    DrmDeviceFd,
>;

type CardOutput = DrmOutput<
    GbmAllocator<DrmDeviceFd>,
    GbmFramebufferExporter<DrmDeviceFd>,
    Option<OutputPresentationFeedback>,
    DrmDeviceFd,
>;

fn configure_pointer(device: &mut smithay::reexports::input::Device, pointer: &crate::config::Pointer) {
    let touchpad = device.config_tap_finger_count() > 0;
    if device.config_accel_is_available() {
        let _ = device.config_accel_set_speed(pointer.speed);
    }
    if device.config_scroll_has_natural_scroll() {
        let natural = if touchpad { pointer.touchpad_natural } else { pointer.mouse_natural };
        let _ = device.config_scroll_set_natural_scroll_enabled(natural);
    }
    if touchpad {
        let _ = device.config_tap_set_enabled(pointer.tap);
        if device.config_dwt_is_available() {
            let _ = device.config_dwt_set_enabled(pointer.quiet_typing);
        }
    } else if device.config_left_handed_is_available() {
        let _ = device.config_left_handed_set(pointer.left_handed);
    }
}

impl AnvilState<UdevData> {

    pub fn apply_input_settings(&mut self) {
        let pointer = self.config.pointer.clone();
        self.backend_data.apply_input_settings(&pointer);
    }
}

struct SurfaceData {
    dh: DisplayHandle,
    device_id: DrmNode,
    render_node: Option<DrmNode>,
    global: Option<GlobalId>,
    drm_output: CardOutput,

    frame_flags: FrameFlags,
    dmabuf_feedback: Option<SurfaceDmabufFeedback>,
    last_presentation_time: Option<Time<Monotonic>>,
    vblank_throttle_timer: Option<RegistrationToken>,

    output: Output,
    connector: connector::Handle,
    serial: Option<String>,

    drm_modes: Vec<DrmMode>,
    mode_infos: Vec<ScreenMode>,

    enabled: bool,

    powered: bool,

    gamma: Option<Vec<u16>>,

    original_gamma: Option<Vec<u16>>,

    locked_frame_queued: bool,

    refused_flips: u32,

    failed_frames: u32,

    lit: bool,

    recovery: crate::modeset_recovery::Backoff,

    still_since: Option<Instant>,

    resting: Option<Instant>,

    rest_timer: bool,

    rest_looks: (u32, u32),
    rest_told: Option<Instant>,
}

impl Drop for SurfaceData {
    fn drop(&mut self) {
        if let Some(global) = self.global.take() {
            self.dh.remove_global::<AnvilState<UdevData>>(global);
        }
    }
}

struct BackendData {
    surfaces: HashMap<crtc::Handle, SurfaceData>,
    non_desktop_connectors: Vec<(connector::Handle, crtc::Handle)>,
    leasing_global: Option<DrmLeaseState>,
    active_leases: Vec<DrmLease>,
    drm_output_manager: OutputManager,
    drm_scanner: DrmScanner,
    render_node: Option<DrmNode>,
    registration_token: RegistrationToken,

    clean_start: crate::clean_start::CleanStart,
}

#[derive(Debug, thiserror::Error)]
enum DeviceAddError {
    #[error("Failed to open device using libseat: {0}")]
    DeviceOpen(libseat::Error),
    #[error("Failed to initialize drm device: {0}")]
    DrmDevice(DrmError),
    #[error("Failed to initialize gbm device: {0}")]
    GbmDevice(std::io::Error),
    #[error("Failed to access drm node: {0}")]
    DrmNode(CreateDrmNodeError),
    #[error("Failed to add device to GpuManager: {0}")]
    AddNode(egl::Error),
    #[error("Primary GPU is missing")]
    PrimaryGpuMissing,
}
