use crate::{
    backend::renderer::element::{Id, RenderElementPresentationState, RenderElementStates, RenderingReason},
    wayland::dmabuf::DmabufFeedback,
};

pub fn select_dmabuf_feedback<'a>(
    element: impl Into<Id>,
    render_element_states: &RenderElementStates,
    default_feedback: &'a DmabufFeedback,
    scanout_feedback: &'a DmabufFeedback,
) -> &'a DmabufFeedback {
    let id = element.into();

    let Some(state) = render_element_states.element_render_state(id) else {
        return default_feedback;
    };

    match state.presentation_state {
        RenderElementPresentationState::Rendering { reason } => match reason {
            Some(RenderingReason::FormatUnsupported) | Some(RenderingReason::ScanoutFailed) => {
                scanout_feedback
            }
            None => default_feedback,
        },
        RenderElementPresentationState::ZeroCopy => scanout_feedback,
        RenderElementPresentationState::Skipped => default_feedback,
    }
}
