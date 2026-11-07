use std::mem;
use std::sync::{Arc, Mutex};

use tracing::debug;
use wayland_protocols::wp::text_input::zv3::server::zwp_text_input_v3::{
    self, ChangeCause, ContentHint, ContentPurpose, ZwpTextInputV3,
};
use wayland_server::backend::{ClientId, ObjectId};
use wayland_server::{protocol::wl_surface::WlSurface, Dispatch, Resource};

use crate::input::SeatHandler;
use crate::utils::{Logical, Rectangle};
use crate::wayland::input_method::InputMethodHandle;

use super::TextInputManagerState;

#[derive(Default, Debug)]
pub(crate) struct TextInput {
    instances: Vec<Instance>,
    focus: Option<WlSurface>,
    active_text_input_id: Option<ObjectId>,
}

impl TextInput {
    fn with_focused_client_all_text_inputs<F>(&mut self, mut f: F)
    where
        F: FnMut(&ZwpTextInputV3, &WlSurface, u32),
    {
        if let Some(surface) = self.focus.as_ref().filter(|surface| surface.is_alive()) {
            for text_input in self.instances.iter() {
                let instance_id = text_input.instance.id();
                if instance_id.same_client_as(&surface.id()) {
                    f(&text_input.instance, surface, text_input.serial);
                    break;
                }
            }
        };
    }

    fn with_active_text_input<F>(&mut self, mut f: F)
    where
        F: FnMut(&ZwpTextInputV3, &WlSurface, u32),
    {
        let active_id = match &self.active_text_input_id {
            Some(active_text_input_id) => active_text_input_id,
            None => return,
        };

        let surface = match self.focus.as_ref().filter(|surface| surface.is_alive()) {
            Some(surface) => surface,
            None => return,
        };

        let surface_id = surface.id();
        if let Some(text_input) = self
            .instances
            .iter()
            .filter(|instance| instance.instance.id().same_client_as(&surface_id))
            .find(|instance| &instance.instance.id() == active_id)
        {
            f(&text_input.instance, surface, text_input.serial);
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct TextInputHandle {
    pub(crate) inner: Arc<Mutex<TextInput>>,
}

impl TextInputHandle {
    pub(super) fn add_instance(&self, instance: &ZwpTextInputV3) {
        let mut inner = self.inner.lock().unwrap();
        inner.instances.push(Instance {
            instance: instance.clone(),
            serial: 0,
            pending_state: Default::default(),
            seen: Seen::default(),
        });
    }

    fn increment_serial(&self, text_input: &ZwpTextInputV3) {
        if let Some(instance) = self
            .inner
            .lock()
            .unwrap()
            .instances
            .iter_mut()
            .find(|instance| instance.instance == *text_input)
        {
            instance.serial += 1
        }
    }

    pub fn focus(&self) -> Option<WlSurface> {
        self.inner.lock().unwrap().focus.clone()
    }

    pub fn set_focus(&self, surface: Option<WlSurface>) {
        self.inner.lock().unwrap().focus = surface;
    }

    pub fn leave(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.active_text_input_id = None;
        inner.with_focused_client_all_text_inputs(|text_input, focus, _| {
            text_input.leave(focus);
        });
    }

    pub fn enter(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.with_focused_client_all_text_inputs(|text_input, focus, _| {
            text_input.enter(focus);
        });
    }

    pub fn done(&self, discard_state: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.with_active_text_input(|text_input, _, serial| {
            if discard_state {
                debug!("discarding text-input state due to serial");
                text_input.done(0);
            } else {
                text_input.done(serial);
            }
        });
    }

    pub fn active_purpose(&self) -> Option<ContentPurpose> {
        let inner = self.inner.lock().unwrap();
        let focus = inner.focus.as_ref().filter(|s| s.is_alive())?.id();
        let secret = |p: &ContentPurpose| matches!(p, ContentPurpose::Password | ContentPurpose::Pin);
        inner.instances.iter()
            .filter(|i| i.seen.enabled && i.instance.id().same_client_as(&focus))
            .filter_map(|i| i.seen.purpose)
            .max_by_key(secret)
    }

    pub fn active_sensitive(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        let Some(focus) = inner.focus.as_ref().filter(|s| s.is_alive()).map(|s| s.id()) else { return false };
        inner.instances.iter().any(|i| i.seen.enabled && i.seen.sensitive && i.instance.id().same_client_as(&focus))
    }

    pub fn with_focused_text_input<F>(&self, mut f: F)
    where
        F: FnMut(&ZwpTextInputV3, &WlSurface),
    {
        let mut inner = self.inner.lock().unwrap();
        inner.with_focused_client_all_text_inputs(|ti, surface, _| {
            f(ti, surface);
        });
    }

    pub fn with_active_text_input<F>(&self, mut f: F)
    where
        F: FnMut(&ZwpTextInputV3, &WlSurface),
    {
        let mut inner = self.inner.lock().unwrap();
        inner.with_active_text_input(|ti, surface, _| {
            f(ti, surface);
        });
    }

    pub(crate) fn active_text_input_serial_or_default<F>(&self, default: u32, mut callback: F)
    where
        F: FnMut(u32),
    {
        let mut inner = self.inner.lock().unwrap();
        let mut should_default = true;
        inner.with_active_text_input(|_, _, serial| {
            should_default = false;
            callback(serial);
        });
        if should_default {
            callback(default)
        }
    }
}

#[derive(Debug)]
pub struct TextInputUserData {
    pub(super) handle: TextInputHandle,
    pub(crate) input_method_handle: InputMethodHandle,
}

impl<D> Dispatch<ZwpTextInputV3, TextInputUserData, D> for TextInputManagerState
where
    D: Dispatch<ZwpTextInputV3, TextInputUserData>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        state: &mut D,
        _client: &wayland_server::Client,
        resource: &ZwpTextInputV3,
        request: zwp_text_input_v3::Request,
        data: &TextInputUserData,
        _dhandle: &wayland_server::DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        if matches!(request, zwp_text_input_v3::Request::Commit) {
            data.handle.increment_serial(resource);
        }

        if let Some(i) = data.handle.inner.lock().unwrap().instances.iter_mut().find(|i| i.instance == *resource) {
            let seen = &mut i.seen;
            match &request {
                zwp_text_input_v3::Request::Enable => {
                    seen.pending_enable = Some(true);
                    seen.pending_purpose = Some(ContentPurpose::Normal);
                    seen.pending_sensitive = Some(false);
                }
                zwp_text_input_v3::Request::Disable => seen.pending_enable = Some(false),
                zwp_text_input_v3::Request::SetContentType { hint, purpose } => {
                    seen.pending_purpose = purpose.into_result().ok();
                    seen.pending_sensitive = hint.into_result().ok().map(|h| h.contains(ContentHint::SensitiveData));
                }
                zwp_text_input_v3::Request::Commit => {
                    if let Some(on) = seen.pending_enable.take() {
                        seen.enabled = on;
                    }
                    if let Some(p) = seen.pending_purpose.take() {
                        seen.purpose = Some(p);
                    }
                    if let Some(h) = seen.pending_sensitive.take() {
                        seen.sensitive = h;
                    }
                }
                _ => {}
            }
        }

        if !data.input_method_handle.has_instance() {
            debug!("discarding text-input request without IME running");
            return;
        }

        let focus = match data.handle.focus() {
            Some(focus) if focus.id().same_client_as(&resource.id()) => focus,
            _ => {
                debug!("discarding text-input request for unfocused client");
                return;
            }
        };

        let mut guard = data.handle.inner.lock().unwrap();
        let pending_state = match guard.instances.iter_mut().find_map(|instance| {
            if instance.instance == *resource {
                Some(&mut instance.pending_state)
            } else {
                None
            }
        }) {
            Some(pending_state) => pending_state,
            None => {
                debug!("got request for untracked text-input");
                return;
            }
        };

        match request {
            zwp_text_input_v3::Request::Enable => {
                pending_state.enable = Some(true);
            }
            zwp_text_input_v3::Request::Disable => {
                pending_state.enable = Some(false);
            }
            zwp_text_input_v3::Request::SetSurroundingText { text, cursor, anchor } => {
                pending_state.surrounding_text = Some((text, cursor as u32, anchor as u32));
            }
            zwp_text_input_v3::Request::SetTextChangeCause { cause } => {
                pending_state.text_change_cause = Some(cause.into_result().unwrap());
            }
            zwp_text_input_v3::Request::SetContentType { hint, purpose } => {
                pending_state.content_type =
                    Some((hint.into_result().unwrap(), purpose.into_result().unwrap()));
            }
            zwp_text_input_v3::Request::SetCursorRectangle { x, y, width, height } => {
                pending_state.cursor_rectangle = Some(Rectangle::new((x, y).into(), (width, height).into()));
            }
            zwp_text_input_v3::Request::Commit => {
                let mut new_state = mem::take(pending_state);
                let _ = pending_state;
                let active_text_input_id = &mut guard.active_text_input_id;

                if active_text_input_id.is_some() && *active_text_input_id != Some(resource.id()) {
                    debug!("discarding text_input request since we already have an active one");
                    return;
                }

                match new_state.enable {
                    Some(true) => {
                        *active_text_input_id = Some(resource.id());
                        drop(guard);
                        data.input_method_handle.activate_input_method(state, &focus);
                    }
                    Some(false) => {
                        *active_text_input_id = None;
                        drop(guard);
                        data.input_method_handle.deactivate_input_method(state);
                        return;
                    }
                    None => {
                        if *active_text_input_id != Some(resource.id()) {
                            debug!("discarding text_input requests before enabling it");
                            return;
                        }

                        drop(guard);
                    }
                }

                if let Some((text, cursor, anchor)) = new_state.surrounding_text.take() {
                    data.input_method_handle.with_instance(move |input_method| {
                        input_method.object.surrounding_text(text, cursor, anchor)
                    });
                }

                if let Some(cause) = new_state.text_change_cause.take() {
                    data.input_method_handle.with_instance(move |input_method| {
                        input_method.object.text_change_cause(cause);
                    });
                }

                if let Some((hint, purpose)) = new_state.content_type.take() {
                    data.input_method_handle.with_instance(move |input_method| {
                        input_method.object.content_type(hint, purpose);
                    });
                }

                if let Some(rect) = new_state.cursor_rectangle.take() {
                    data.input_method_handle
                        .set_text_input_rectangle::<D>(state, rect);
                }

                data.input_method_handle.with_instance(|input_method| {
                    input_method.done();
                });
            }
            zwp_text_input_v3::Request::Destroy => {
            }
            _ => unreachable!(),
        }
    }

    fn destroyed(state: &mut D, _client: ClientId, text_input: &ZwpTextInputV3, data: &TextInputUserData) {
        let destroyed_id = text_input.id();
        let deactivate_im = {
            let mut inner = data.handle.inner.lock().unwrap();
            inner.instances.retain(|inst| inst.instance.id() != destroyed_id);
            let destroyed_focused = inner
                .focus
                .as_ref()
                .map(|focus| focus.id().same_client_as(&destroyed_id))
                .unwrap_or(true);

            destroyed_focused
                && !inner
                    .instances
                    .iter()
                    .any(|inst| inst.instance.id().same_client_as(&destroyed_id))
        };

        if deactivate_im {
            data.input_method_handle.deactivate_input_method(state);
        }
    }
}

#[derive(Debug)]
struct Instance {
    instance: ZwpTextInputV3,
    serial: u32,
    pending_state: TextInputState,
    seen: Seen,
}

#[derive(Debug, Default)]
struct Seen {
    pending_enable: Option<bool>,
    pending_purpose: Option<ContentPurpose>,
    pending_sensitive: Option<bool>,
    enabled: bool,
    purpose: Option<ContentPurpose>,
    sensitive: bool,
}

#[derive(Debug, Default)]
struct TextInputState {
    enable: Option<bool>,
    surrounding_text: Option<(String, u32, u32)>,
    content_type: Option<(ContentHint, ContentPurpose)>,
    cursor_rectangle: Option<Rectangle<i32, Logical>>,
    text_change_cause: Option<ChangeCause>,
}
