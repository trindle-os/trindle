// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Sharing graphics buffers: what the dmabuf global tells programs about the cards, and explicit sync.

use smithay::{
    backend::{
        allocator::{dmabuf::Dmabuf, format::FormatSet},
        drm::{DrmDeviceFd, DrmNode, DrmSurface, NodeType},
        renderer::{
            gles::GlesRenderer,
            multigpu::{gbm::GbmGlesBackend, GpuManager},
            ImportDma,
        },
    },
    delegate_dmabuf,
    reexports::wayland_protocols::wp::linux_dmabuf::zv1::server::zwp_linux_dmabuf_feedback_v1,
    wayland::{
        dmabuf::{DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState, ImportNotifier},
        drm_syncobj::{supports_syncobj_eventfd, DrmSyncobjHandler, DrmSyncobjState},
    },
};

use super::{AnvilState, UdevData};
use crate::state::SurfaceDmabufFeedback;

impl DmabufHandler for AnvilState<UdevData> {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        &mut self.backend_data.dmabuf_state.as_mut().unwrap().0
    }

    fn dmabuf_imported(&mut self, _global: &DmabufGlobal, dmabuf: Dmabuf, notifier: ImportNotifier) {
        let primary = self.backend_data.primary_gpu;
        let mut nodes = vec![primary];
        for node in self.backend_data.backends.values().filter_map(|b| b.render_node) {
            if !nodes.contains(&node) {
                nodes.push(node);
            }
        }
        let taken = nodes.into_iter().find(|node| {
            self.backend_data
                .gpus
                .single_renderer(node)
                .and_then(|mut renderer| renderer.import_dmabuf(&dmabuf, None))
                .is_ok()
        });
        if let Some(node) = taken {
            dmabuf.set_node(node);
            let _ = notifier.successful::<AnvilState<UdevData>>();
        } else {
            notifier.failed();
        }
    }
}
delegate_dmabuf!(AnvilState<UdevData>);

impl DrmSyncobjHandler for AnvilState<UdevData> {
    fn drm_syncobj_state(&mut self) -> Option<&mut DrmSyncobjState> {
        self.backend_data.syncobj_state.as_mut()
    }
}
smithay::delegate_drm_syncobj!(AnvilState<UdevData>);

impl AnvilState<UdevData> {
    pub(super) fn default_dmabuf_feedback(&mut self) -> Option<DmabufFeedback> {
        let primary = self.backend_data.primary_gpu;
        let formats = self.backend_data.gpus.single_renderer(&primary).ok()?.dmabuf_formats();
        let mut builder = DmabufFeedbackBuilder::new(primary.dev_id(), formats.clone())
            .add_preference_tranche(primary.dev_id(), None, formats);
        let others: std::collections::HashSet<_> = self
            .backend_data
            .backends
            .values()
            .filter_map(|b| b.render_node)
            .filter(|n| *n != primary)
            .collect();
        for node in others {
            if let Ok(renderer) = self.backend_data.gpus.single_renderer(&node) {
                builder = builder.add_preference_tranche(node.dev_id(), None, renderer.dmabuf_formats());
            }
        }
        builder.build().ok()
    }

    pub(super) fn check_syncobj(&mut self) {
        if self.backend_data.syncobj_state.is_some() {
            return;
        }
        let Some(primary_node) = self.backend_data.primary_gpu.node_with_type(NodeType::Primary).and_then(|x| x.ok())
        else {
            return;
        };
        let Some(backend) = self.backend_data.backends.get(&primary_node) else { return };
        let import_device = backend.drm_output_manager.device().device_fd().clone();
        if supports_syncobj_eventfd(&import_device) {
            self.backend_data.syncobj_state =
                Some(DrmSyncobjState::new::<AnvilState<UdevData>>(&self.display_handle, import_device));
        }
    }

    pub(super) fn resend_default_dmabuf_feedback(&mut self) {
        if self.backend_data.dmabuf_state.is_none() {
            return;
        }
        let Some(feedback) = self.default_dmabuf_feedback() else { return };
        if let Some((state, global)) = self.backend_data.dmabuf_state.as_mut() {
            state.set_default_feedback(global, &feedback);
        }
    }

    pub(super) fn early_release_if_one_gpu(&mut self) {
        let gpus: std::collections::HashSet<_> =
            self.backend_data.backends.values().filter_map(|b| b.render_node).collect();
        smithay::backend::renderer::utils::set_early_shm_release(gpus.len() <= 1);
    }
}

pub(super) fn get_surface_dmabuf_feedback(
    primary_gpu: DrmNode,
    render_node: Option<DrmNode>,
    scanout_node: DrmNode,
    gpus: &mut GpuManager<GbmGlesBackend<GlesRenderer, DrmDeviceFd>>,
    surface: &DrmSurface,
) -> Option<SurfaceDmabufFeedback> {
    let primary_formats = gpus.single_renderer(&primary_gpu).ok()?.dmabuf_formats();
    let render_formats = if let Some(render_node) = render_node {
        gpus.single_renderer(&render_node).ok()?.dmabuf_formats()
    } else {
        FormatSet::default()
    };

    let all_render_formats = primary_formats
        .iter()
        .chain(render_formats.iter())
        .copied()
        .collect::<FormatSet>();
    let planes_formats = scanout_formats(surface, &all_render_formats);

    let builder = DmabufFeedbackBuilder::new(primary_gpu.dev_id(), primary_formats);
    let render_feedback = if let Some(render_node) = render_node {
        builder
            .clone()
            .add_preference_tranche(render_node.dev_id(), None, render_formats.clone())
            .build()
            .unwrap()
    } else {
        builder.clone().build().unwrap()
    };

    let scanout_feedback = builder
        .add_preference_tranche(
            surface.device_fd().dev_id().unwrap(),
            Some(zwp_linux_dmabuf_feedback_v1::TrancheFlags::Scanout),
            planes_formats,
        )
        .add_preference_tranche(scanout_node.dev_id(), None, render_formats)
        .build()
        .unwrap();

    Some(SurfaceDmabufFeedback {
        render_feedback,
        scanout_feedback,
    })
}

fn scanout_formats(surface: &DrmSurface, all_render_formats: &FormatSet) -> FormatSet {
    let planes = surface.planes().clone();
    surface
        .plane_info()
        .formats
        .iter()
        .copied()
        .chain(planes.overlay.into_iter().flat_map(|p| p.formats))
        .collect::<FormatSet>()
        .intersection(all_render_formats)
        .copied()
        .collect::<FormatSet>()
}
