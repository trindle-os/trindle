// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Graphics cards: choosing the main one, opening and dropping cards, and keeping their screens in step with what is plugged in.

use std::{collections::hash_map::HashMap, path::Path};

use smithay::{
    backend::{
        allocator::{
            format::FormatSet,
            gbm::{GbmAllocator, GbmBufferFlags, GbmDevice},
            Fourcc, Modifier,
        },
        drm::{
            exporter::gbm::GbmFramebufferExporter, output::DrmOutputManager, DrmDevice, DrmDeviceFd,
            DrmDeviceNotifier, DrmEvent, DrmNode, NodeType,
        },
        egl::{EGLDevice, EGLDisplay},
        renderer::ImportMemWl,
        session::{libseat::LibSeatSession, Session},
        udev::{all_gpus, primary_gpu, UdevBackend},
    },
    reexports::{calloop::RegistrationToken, rustix::fs::OFlags},
    utils::DeviceFd,
    wayland::drm_lease::DrmLeaseState,
};
use tracing::{debug, error, info, warn};

use super::{AnvilState, BackendData, DeviceAddError, OutputManager, UdevData};
use crate::{
    clean_start::CleanStart,
    drm_extras::{drm_scanner::{DrmScanEvent, DrmScanner}, leftovers},
};

const SUPPORTED_FORMATS: &[Fourcc] = &[
    Fourcc::Abgr2101010,
    Fourcc::Argb2101010,
    Fourcc::Abgr8888,
    Fourcc::Argb8888,
];
const SUPPORTED_FORMATS_8BIT_ONLY: &[Fourcc] = &[Fourcc::Abgr8888, Fourcc::Argb8888];

pub(super) fn choose_primary_gpu(session: &LibSeatSession) -> Option<DrmNode> {
    let chosen = std::env::var("ANVIL_DRM_DEVICE").ok().and_then(|var| match DrmNode::from_path(&var) {
        Ok(node) => Some(node),
        Err(err) => {
            warn!(?err, "ANVIL_DRM_DEVICE {} is not a graphics card, choosing one instead", var);
            None
        }
    });
    chosen
        .or_else(|| panel_gpu(&session.seat()))
        .or_else(|| {
            primary_gpu(session.seat()).ok().flatten()
                .and_then(|x| DrmNode::from_path(x).ok()?.node_with_type(NodeType::Render)?.ok())
        })
        .or_else(|| {
            all_gpus(session.seat()).unwrap_or_default().into_iter().find_map(|x| DrmNode::from_path(x).ok())
        })
}

fn panel_gpu(seat: &str) -> Option<DrmNode> {
    let panel = |card: &str| {
        let Ok(entries) = std::fs::read_dir("/sys/class/drm") else { return false };
        entries.flatten().any(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let built_in = name
                .strip_prefix(card)
                .and_then(|rest| rest.strip_prefix('-'))
                .is_some_and(|c| ["eDP", "LVDS", "DSI"].iter().any(|k| c.starts_with(k)));
            built_in && std::fs::read_to_string(e.path().join("status")).is_ok_and(|s| s.trim() == "connected")
        })
    };
    let gpus = all_gpus(seat).ok()?;
    if gpus.len() < 2 {
        return None;
    }
    gpus.into_iter().find_map(|path| {
        let card = path.file_name()?.to_str()?.to_string();
        if !panel(&card) {
            return None;
        }
        DrmNode::from_path(&path).ok()?.node_with_type(NodeType::Render)?.ok()
    })
}

impl AnvilState<UdevData> {
    pub(super) fn device_added(&mut self, node: DrmNode, path: &Path) -> Result<(), DeviceAddError> {
        let fd = self
            .backend_data
            .session
            .open(
                path,
                OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK,
            )
            .map_err(DeviceAddError::DeviceOpen)?;

        let fd = DrmDeviceFd::new(DeviceFd::from(fd));

        let (drm, notifier) = DrmDevice::new(fd.clone(), false).map_err(DeviceAddError::DrmDevice)?;
        if let Err(err) = leftovers::clear(&drm) {
            warn!(?err, "could not switch off what was left lit on {}", node);
        }
        let gbm = GbmDevice::new(fd).map_err(DeviceAddError::GbmDevice)?;

        let registration_token = self.watch_vblanks(node, notifier);

        let (render_node, software) = match self.init_gpu(node, &gbm) {
            Ok((node, software)) => (Some(node), software),
            Err(err) => {
                warn!(?err, "failed to initialize gpu");
                (None, false)
            }
        };

        let allocator = self.card_allocator(render_node, &gbm)?;
        let framebuffer_exporter = GbmFramebufferExporter::new(gbm.clone(), render_node);
        let render_formats = self.card_render_formats(render_node)?;
        let color_formats = if std::env::var("ANVIL_DISABLE_10BIT").is_ok() {
            SUPPORTED_FORMATS_8BIT_ONLY
        } else {
            SUPPORTED_FORMATS
        };
        let drm_output_manager = DrmOutputManager::new(
            drm,
            allocator,
            framebuffer_exporter,
            Some(gbm),
            color_formats.iter().copied(),
            render_formats,
        );

        self.add_backend(node, registration_token, drm_output_manager, render_node, software);
        self.early_release_if_one_gpu();
        self.resend_default_dmabuf_feedback();
        self.check_syncobj();

        self.device_changed(node);

        Ok(())
    }

    fn watch_vblanks(&mut self, node: DrmNode, notifier: DrmDeviceNotifier) -> RegistrationToken {
        self.handle
            .insert_source(
                notifier,
                move |event, metadata, data: &mut AnvilState<_>| match event {
                    DrmEvent::VBlank(crtc) => {
                        data.frame_finish(node, crtc, metadata);
                    }
                    DrmEvent::Error(error) => {
                        error!("{:?}", error);
                    }
                },
            )
            .unwrap()
    }

    fn init_gpu(&mut self, node: DrmNode, gbm: &GbmDevice<DrmDeviceFd>) -> Result<(DrmNode, bool), DeviceAddError> {
        // SAFETY: the GBM device is kept alive by this backend for as
        // long as the EGL display made from it.
        let display = unsafe { EGLDisplay::new(gbm.clone()).map_err(DeviceAddError::AddNode)? };
        let egl_device = EGLDevice::device_for_display(&display).map_err(DeviceAddError::AddNode)?;

        let software = egl_device.is_software();
        if software {
            info!(?node, "no 3D on this card: the processor draws, and windows do not move");
            crate::animate::disable();
            crate::rest::drawn_by_processor();
            std::env::set_var("TRINDLE_DRAWN_BY_PROCESSOR", "1");
        }
        let render_node = egl_device.try_get_render_node().ok().flatten().unwrap_or(node);
        let primary = self.backend_data.primary_gpu;
        let same_card = primary == node
            || node.node_with_type(NodeType::Render).and_then(|n| n.ok()).is_some_and(|n| n == primary);
        let main_missing = !same_card
            && !self.backend_data.backends.values().any(|b| b.render_node == Some(primary));
        if (software && same_card) || main_missing {
            self.backend_data.primary_gpu = render_node;
        }
        self.backend_data
            .gpus
            .as_mut()
            .add_node(render_node, gbm.clone())
            .map_err(DeviceAddError::AddNode)?;

        Ok((render_node, software))
    }

    fn card_allocator(
        &self,
        render_node: Option<DrmNode>,
        gbm: &GbmDevice<DrmDeviceFd>,
    ) -> Result<GbmAllocator<DrmDeviceFd>, DeviceAddError> {
        render_node
            .is_some()
            .then(|| GbmAllocator::new(gbm.clone(), GbmBufferFlags::RENDERING | GbmBufferFlags::SCANOUT))
            .or_else(|| {
                self.backend_data
                    .backends
                    .get(&self.backend_data.primary_gpu)
                    .or_else(|| {
                        self.backend_data
                            .backends
                            .values()
                            .find(|backend| backend.render_node == Some(self.backend_data.primary_gpu))
                    })
                    .map(|backend| backend.drm_output_manager.allocator().clone())
            })
            .ok_or(DeviceAddError::PrimaryGpuMissing)
    }

    fn card_render_formats(&mut self, render_node: Option<DrmNode>) -> Result<FormatSet, DeviceAddError> {
        let mut renderer = self
            .backend_data
            .gpus
            .single_renderer(&render_node.unwrap_or(self.backend_data.primary_gpu))
            .map_err(|_| DeviceAddError::PrimaryGpuMissing)?;
        Ok(renderer
            .as_mut()
            .egl_context()
            .dmabuf_render_formats()
            .iter()
            .filter(|format| render_node.is_some() || format.modifier == Modifier::Linear)
            .copied()
            .collect::<FormatSet>())
    }

    fn add_backend(
        &mut self,
        node: DrmNode,
        registration_token: RegistrationToken,
        drm_output_manager: OutputManager,
        render_node: Option<DrmNode>,
        software: bool,
    ) {
        self.backend_data.retired_leases.remove(&node);
        self.backend_data.backends.insert(
            node,
            BackendData {
                registration_token,
                clean_start: CleanStart::default(),
                drm_output_manager,
                drm_scanner: DrmScanner::new(),
                non_desktop_connectors: Vec::new(),
                render_node,
                surfaces: HashMap::new(),
                leasing_global: (!software)
                    .then(|| DrmLeaseState::new::<AnvilState<UdevData>>(&self.display_handle, &node))
                    .and_then(|lease| {
                        lease.inspect_err(|err| {
                            warn!(?err, "Failed to initialize drm lease global for: {}", node);
                        })
                        .ok()
                    }),
                active_leases: Vec::new(),
            },
        );
    }

    pub(super) fn recheck_devices(&mut self, failed: &[DrmNode]) {
        let present: Vec<(DrmNode, std::path::PathBuf)> = match UdevBackend::new(&self.seat_name) {
            Ok(udev) => udev.device_list()
                .filter_map(|(id, path)| Some((DrmNode::from_dev_id(id).ok()?, path.to_path_buf())))
                .collect(),
            Err(err) => {
                warn!(?err, "could not list the graphics cards again");
                let known: Vec<DrmNode> = self.backend_data.backends.keys().copied().collect();
                for node in known {
                    self.device_changed(node);
                    self.render(node, None, self.clock.now());
                }
                return;
            }
        };
        let known: Vec<DrmNode> = self.backend_data.backends.keys().copied().collect();
        for node in known.iter().filter(|n| failed.contains(n) || !present.iter().any(|(p, _)| p == *n)) {
            info!("dropping {} after the switch back", node);
            self.device_removed(*node);
        }
        for (node, path) in &present {
            if self.backend_data.backends.contains_key(node) {
                self.device_changed(*node);
            } else if let Err(err) = self.device_added(*node, path) {
                error!("Skipping device {node}: {err}");
                continue;
            }
            self.render(*node, None, self.clock.now());
        }
    }

    pub(super) fn device_changed(&mut self, node: DrmNode) {
        let device = if let Some(device) = self.backend_data.backends.get_mut(&node) {
            device
        } else {
            return;
        };

        let scan_result = match device
            .drm_scanner
            .scan_connectors(device.drm_output_manager.device())
        {
            Ok(scan_result) => scan_result,
            Err(err) => {
                tracing::warn!(?err, "Failed to scan connectors");
                return;
            }
        };

        for event in scan_result {
            match event {
                DrmScanEvent::Connected {
                    connector,
                    crtc: Some(crtc),
                } => {
                    self.connector_connected(node, connector, crtc);
                }
                DrmScanEvent::Disconnected {
                    connector,
                    crtc: Some(crtc),
                } => {
                    self.connector_disconnected(node, connector, crtc);
                }
                DrmScanEvent::Connected { connector, crtc: None } => {
                    warn!("{}-{} is plugged in, but the graphics card has no pipe free to drive it",
                          connector.interface().as_str(), connector.interface_id());
                }
                _ => {}
            }
        }

        crate::shell::fixup_positions(&mut self.space, self.pointer.current_location());
        self.refresh_screens();
    }

    pub(super) fn device_removed(&mut self, node: DrmNode) {
        let device = if let Some(device) = self.backend_data.backends.get_mut(&node) {
            device
        } else {
            return;
        };

        let crtcs: Vec<_> = device
            .drm_scanner
            .crtcs()
            .map(|(info, crtc)| (info.clone(), crtc))
            .collect();

        for (connector, crtc) in crtcs {
            self.connector_disconnected(node, connector, crtc);
        }

        debug!("Surfaces dropped");

        if let Some(mut backend_data) = self.backend_data.backends.remove(&node) {
            if let Some(mut leasing_global) = backend_data.leasing_global.take() {
                leasing_global.disable_global::<AnvilState<UdevData>>();
                self.backend_data.retired_leases.insert(node, leasing_global);
            }

            if let Some(render_node) = backend_data.render_node {
                let still_used = self.backend_data.backends.values().any(|b| b.render_node == Some(render_node));
                if render_node == self.backend_data.primary_gpu && !still_used {
                    let next = self.backend_data.backends.values().find_map(|b| b.render_node);
                    if let Some(next) = next {
                        info!(?next, "the main card went; this one takes over");
                        self.backend_data.primary_gpu = next;
                        if let Ok(renderer) = self.backend_data.gpus.single_renderer(&next) {
                            self.shm_state.update_formats(renderer.shm_formats());
                        }
                    }
                }
                if !still_used {
                    self.backend_data.gpus.as_mut().remove_node(&render_node);
                }
            }

            self.handle.remove(backend_data.registration_token);

            debug!("Dropping device");
        }
        self.early_release_if_one_gpu();
        self.resend_default_dmabuf_feedback();

        crate::shell::fixup_positions(&mut self.space, self.pointer.current_location());
        self.refresh_screens();
    }
}
