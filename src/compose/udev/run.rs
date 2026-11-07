// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Starting the udev backend and running its event loop: session, cards, input devices and the loop itself.

use std::{
    collections::hash_map::HashMap,
    sync::atomic::Ordering,
    time::Duration,
};

use smithay::{
    backend::{
        drm::{DrmNode, NodeType},
        egl::context::ContextPriority,
        input::InputEvent,
        libinput::{LibinputInputBackend, LibinputSessionInterface},
        renderer::{
            multigpu::{gbm::GbmGlesBackend, GpuManager},
            ImportEgl, ImportMemWl,
        },
        session::{libseat::LibSeatSession, Event as SessionEvent, Session},
        udev::{UdevBackend, UdevEvent},
    },
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop, LoopHandle,
        },
        input::{Device, DeviceCapability, Libinput},
        wayland_server::{Display, DisplayHandle},
    },
    wayland::dmabuf::DmabufState,
};
use tracing::{error, info};

use super::{
    cards::choose_primary_gpu, configure_pointer, console_switch, dmabuf::get_surface_dmabuf_feedback, AnvilState,
    DeviceAddError, UdevData,
};
use crate::drawing::PointerElement;

type Loop = EventLoop<'static, AnvilState<UdevData>>;

pub fn run_udev() {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display = Display::new().unwrap();
    let mut display_handle = display.handle();

    let (session, notifier) = match LibSeatSession::new() {
        Ok(ret) => ret,
        Err(err) => {
            error!("Could not initialize a session: {}", err);
            return;
        }
    };

    let Some(primary_gpu) = choose_primary_gpu(&session) else {
        error!("No graphics card found, so there is nothing to draw on");
        return;
    };
    info!("Using {} as primary gpu.", primary_gpu);

    let mut state = new_state(display, &event_loop, session, primary_gpu);

    let udev_backend = match UdevBackend::new(&state.seat_name) {
        Ok(ret) => ret,
        Err(err) => {
            error!(error = ?err, "Failed to initialize udev backend");
            return;
        }
    };

    let (libinput_context, libinput_backend) = open_libinput(&state);

    watch_input(&event_loop.handle(), libinput_backend);
    watch_session(&event_loop.handle(), notifier, libinput_context);

    if !open_cards(&mut state, &udev_backend, primary_gpu) {
        return;
    }
    let primary_gpu = state.backend_data.primary_gpu;
    share_graphics(&mut state, &display_handle, primary_gpu);
    state.check_syncobj();
    watch_cards(&event_loop.handle(), udev_backend);

    start_services(&mut state, &event_loop);
    run_loop(&mut event_loop, &mut state, &mut display_handle);
}

fn new_state(
    display: Display<AnvilState<UdevData>>,
    event_loop: &Loop,
    session: LibSeatSession,
    primary_gpu: DrmNode,
) -> AnvilState<UdevData> {
    let gpus = GpuManager::new(GbmGlesBackend::with_context_priority(ContextPriority::High)).unwrap();

    let data = UdevData {
        dh: display.handle(),
        dmabuf_state: None,
        syncobj_state: None,
        session,
        primary_gpu,
        gpus,
        backends: HashMap::new(),
        retired_leases: HashMap::new(),
        pointer_image: crate::cursor::Cursor::load(),
        pointer_images: Vec::new(),
        pointer_element: PointerElement::default(),
        keyboards: Vec::new(),
        pointers: Vec::new(),
        newcomers: crate::newcomers::Newcomers::load(),
    };
    AnvilState::init(display, event_loop.handle(), data, true)
}

fn open_libinput(state: &AnvilState<UdevData>) -> (Libinput, LibinputInputBackend) {
    let mut libinput_context = Libinput::new_with_udev::<LibinputSessionInterface<LibSeatSession>>(
        state.backend_data.session.clone().into(),
    );
    libinput_context.udev_assign_seat(&state.seat_name).unwrap();
    let libinput_backend = LibinputInputBackend::new(libinput_context.clone());
    (libinput_context, libinput_backend)
}

fn watch_input(handle: &LoopHandle<'static, AnvilState<UdevData>>, libinput_backend: LibinputInputBackend) {
    handle
        .insert_source(libinput_backend, move |mut event, _, data| {
            let dh = data.backend_data.dh.clone();
            if let InputEvent::DeviceAdded { device } = &mut event {
                device_plugged_in(data, device);
            } else if let InputEvent::DeviceRemoved { ref device } = event {
                device_unplugged(data, device);
            }

            data.process_input_event(&dh, event)
        })
        .unwrap();
}

fn device_plugged_in(data: &mut AnvilState<UdevData>, device: &mut Device) {
    if [DeviceCapability::Keyboard, DeviceCapability::Pointer, DeviceCapability::Touch]
        .into_iter()
        .any(|c| device.has_capability(c))
    {
        let arrival = crate::newcomers::Arrival {
            id: device.sysname().to_string(),
            key: crate::newcomers::key_of(
                Some((device.id_vendor(), device.id_product())),
                device.name(),
            ),
            types: device.keyboard_has_key(30) == Ok(true),
        };
        let locked = data.is_locked();
        data.backend_data.newcomers.added(arrival, locked);
    }
    if device.has_capability(DeviceCapability::Keyboard) {
        if let Some(led_state) = data.seat.get_keyboard().map(|keyboard| keyboard.led_state()) {
            device.led_update(led_state.into());
        }
        data.backend_data.keyboards.push(device.clone());
    }
    if device.has_capability(DeviceCapability::Pointer) {
        configure_pointer(device, &data.config.pointer);
        data.backend_data.pointers.push(device.clone());
    }
}

fn device_unplugged(data: &mut AnvilState<UdevData>, device: &Device) {
    data.backend_data.newcomers.removed(device.sysname());
    if device.has_capability(DeviceCapability::Keyboard) {
        data.backend_data.keyboards.retain(|item| item != device);
    }
    data.backend_data.pointers.retain(|item| item != device);
}

fn watch_session(
    handle: &LoopHandle<'static, AnvilState<UdevData>>,
    notifier: smithay::backend::session::libseat::LibSeatSessionNotifier,
    mut libinput_context: Libinput,
) {
    handle
        .insert_source(notifier, move |event, &mut (), data| match event {
            SessionEvent::PauseSession => console_switch::pause(data, &libinput_context),
            SessionEvent::ActivateSession => console_switch::resume(data, &mut libinput_context),
        })
        .unwrap();
}

fn open_cards(state: &mut AnvilState<UdevData>, udev_backend: &UdevBackend, primary_gpu: DrmNode) -> bool {
    let primary_node = primary_gpu
        .node_with_type(NodeType::Primary)
        .and_then(|node| node.ok());
    let primary_device = udev_backend.device_list().find(|(device_id, _)| {
        primary_node
            .map(|primary_node| *device_id == primary_node.dev_id())
            .unwrap_or(false)
            || *device_id == primary_gpu.dev_id()
    });

    if let Some((device_id, path)) = primary_device {
        if let Err(err) = DrmNode::from_dev_id(device_id)
            .map_err(DeviceAddError::DrmNode)
            .and_then(|node| state.device_added(node, path))
        {
            error!("The main card {device_id} would not open, trying the others: {err}");
        }
    }

    let primary_device_id = primary_device.map(|(device_id, _)| device_id);
    for (device_id, path) in udev_backend.device_list() {
        if Some(device_id) == primary_device_id {
            continue;
        }

        if let Err(err) = DrmNode::from_dev_id(device_id)
            .map_err(DeviceAddError::DrmNode)
            .and_then(|node| state.device_added(node, path))
        {
            error!("Skipping device {device_id}: {err}");
        }
    }
    if state.backend_data.backends.is_empty() {
        error!("No graphics card would open, so there is nothing to draw on");
        return false;
    }
    true
}

fn share_graphics(state: &mut AnvilState<UdevData>, display_handle: &DisplayHandle, primary_gpu: DrmNode) {
    state.shm_state.update_formats(
        state
            .backend_data
            .gpus
            .single_renderer(&primary_gpu)
            .unwrap()
            .shm_formats(),
    );

    let mut renderer = state.backend_data.gpus.single_renderer(&primary_gpu).unwrap();

    {
        info!(?primary_gpu, "Trying to initialize EGL Hardware Acceleration",);
        match renderer.bind_wl_display(display_handle) {
            Ok(_) => info!("EGL hardware-acceleration enabled"),
            Err(err) => info!(?err, "Failed to initialize EGL hardware-acceleration"),
        }
    }

    drop(renderer);
    let default_feedback = state.default_dmabuf_feedback().unwrap();
    let mut dmabuf_state = DmabufState::new();
    let global = dmabuf_state
        .create_global_with_default_feedback::<AnvilState<UdevData>>(display_handle, &default_feedback);
    state.backend_data.dmabuf_state = Some((dmabuf_state, global));
    if let Ok(mut renderer) = state.backend_data.gpus.single_renderer(&primary_gpu) {
        let formats = renderer.as_mut().egl_context().dmabuf_render_formats().clone();
        state
            .image_capture_state
            .set_dmabuf(crate::protocols::image_capture::dmabuf_target(primary_gpu.dev_id(), formats.iter()));
    }

    let gpus = &mut state.backend_data.gpus;
    state
        .backend_data
        .backends
        .iter_mut()
        .for_each(|(node, backend_data)| {
            backend_data.surfaces.values_mut().for_each(|surface_data| {
                surface_data.dmabuf_feedback = surface_data.dmabuf_feedback.take().or_else(|| {
                    surface_data.drm_output.with_compositor(|compositor| {
                        get_surface_dmabuf_feedback(
                            primary_gpu,
                            surface_data.render_node,
                            *node,
                            gpus,
                            compositor.surface(),
                        )
                    })
                });
            });
        });
}

fn watch_cards(handle: &LoopHandle<'static, AnvilState<UdevData>>, udev_backend: UdevBackend) {
    handle
        .insert_source(udev_backend, move |event, _, data| match event {
            _ if !data.backend_data.session.is_active() => {}
            UdevEvent::Added { device_id, path } => {
                if let Err(err) = DrmNode::from_dev_id(device_id)
                    .map_err(DeviceAddError::DrmNode)
                    .and_then(|node| data.device_added(node, &path))
                {
                    error!("Skipping device {device_id}: {err}");
                }
            }
            UdevEvent::Changed { device_id } => {
                if let Ok(node) = DrmNode::from_dev_id(device_id) {
                    data.device_changed(node)
                }
            }
            UdevEvent::Removed { device_id } => {
                if let Ok(node) = DrmNode::from_dev_id(device_id) {
                    data.device_removed(node)
                }
            }
        })
        .unwrap();
}

fn start_services(state: &mut AnvilState<UdevData>, event_loop: &Loop) {
    state.watch_settings();
    state.watch_in_use();
    if crate::session::launch().greeter {
        state.session_ready();
    } else {
        state.start_xwayland();
    }

    let every = Duration::from_secs(crate::memory::TRIM_SECS as u64);
    let _ = event_loop.handle().insert_source(Timer::from_duration(every), move |_, _, _| {
        crate::memory::give_back();
        TimeoutAction::ToDuration(every)
    });
}

fn run_loop(event_loop: &mut Loop, state: &mut AnvilState<UdevData>, display_handle: &mut DisplayHandle) {
    while state.running.load(Ordering::SeqCst) {
        let result = event_loop.dispatch(Some(crate::rest::sleep()), state);
        if result.is_err() {
            state.running.store(false, Ordering::SeqCst);
        } else {
            state.space.refresh();
            state.popups.cleanup();
            state.refresh_panel();
            state.refresh_session_lock();
            state.refresh_idle_inhibit();
            state.refresh_suspended();
            state.look_at_resting();
            display_handle.flush_clients().unwrap();
        }
    }
}
