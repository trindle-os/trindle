//! Common base for elements that can be drawn by a [`Renderer`].

use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "wayland_frontend")]
use wayland_server::{backend::ObjectId, Resource};

use crate::{
    output::{Output, WeakOutput},
    utils::{Buffer as BufferCoords, Physical, Point, Rectangle, Scale, Transform},
};

#[cfg(feature = "wayland_frontend")]
use super::utils::Buffer;
use super::{
    utils::{CommitCounter, DamageSet, OpaqueRegions},
    Renderer,
};

pub mod memory;
pub mod solid;
#[cfg(feature = "wayland_frontend")]
pub mod surface;
pub mod texture;
pub mod utils;

crate::utils::ids::id_gen!(external_id);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Id(InnerId);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum InnerId {
    #[cfg(feature = "wayland_frontend")]
    WaylandResource(ObjectId),
    External(Arc<ExternalId>),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ExternalId(usize);

impl ExternalId {
    fn new() -> Self {
        ExternalId(external_id::next())
    }
}

impl Drop for ExternalId {
    fn drop(&mut self) {
        external_id::remove(self.0);
    }
}

impl Id {
    #[cfg(feature = "wayland_frontend")]
    pub fn from_wayland_resource<R: Resource>(resource: &R) -> Self {
        Id(InnerId::WaylandResource(resource.id()))
    }

    pub fn new() -> Self {
        Id(InnerId::External(Arc::new(ExternalId::new())))
    }
}

#[cfg(feature = "wayland_frontend")]
impl<R: Resource> From<&R> for Id {
    #[inline]
    fn from(resource: &R) -> Self {
        Id::from_wayland_resource(resource)
    }
}

#[derive(Debug, Clone)]
pub enum UnderlyingStorage<'a> {
    #[cfg(feature = "wayland_frontend")]
    Wayland(&'a Buffer),
    Memory(&'a memory::MemoryBuffer),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingReason {
    FormatUnsupported,
    ScanoutFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderElementPresentationState {
    Rendering {
        reason: Option<RenderingReason>,
    },
    ZeroCopy,
    Skipped,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderElementState {
    pub visible_area: usize,
    pub presentation_state: RenderElementPresentationState,
}

impl RenderElementState {
    pub(crate) fn skipped() -> Self {
        RenderElementState {
            visible_area: Default::default(),
            presentation_state: RenderElementPresentationState::Skipped,
        }
    }

    pub(crate) fn rendered(visible_area: usize) -> Self {
        RenderElementState {
            visible_area,
            presentation_state: RenderElementPresentationState::Rendering { reason: None },
        }
    }
}

#[derive(Debug, Default)]
pub struct PrimaryScanoutOutput(Option<(WeakOutput, RenderElementState)>);

impl PrimaryScanoutOutput {
    pub fn update_from_render_element_states<F>(
        &mut self,
        element_id: impl Into<Id>,
        output: &Output,
        states: &RenderElementStates,
        compare: F,
    ) -> Option<Output>
    where
        F: for<'a> Fn(&'a Output, &'a RenderElementState, &'a Output, &'a RenderElementState) -> &'a Output,
    {
        let element_id = element_id.into();
        let element_was_presented = states.element_was_presented(element_id.clone());
        let element_state = states.element_render_state(element_id);
        let primary_scanout_output = &mut self.0;

        let has_valid_output = primary_scanout_output
            .as_ref()
            .map(|(current, _)| current.is_alive())
            .unwrap_or(false);
        let same_output = primary_scanout_output
            .as_ref()
            .map(|(current, _)| current == output)
            .unwrap_or(false);

        if !element_was_presented && !has_valid_output {
            return None;
        }

        if !element_was_presented && same_output {
            *primary_scanout_output = None;
            return None;
        }

        if element_was_presented && !has_valid_output {
            *primary_scanout_output = Some((output.downgrade(), element_state.unwrap()));
            return Some(output.clone());
        }

        if element_was_presented && has_valid_output && same_output {
            primary_scanout_output.as_mut().unwrap().1 = element_state.unwrap();
            return Some(output.clone());
        }

        if element_was_presented && has_valid_output && !same_output {
            let (current_output, current_state) = primary_scanout_output
                .as_ref()
                .map(|(output, state)| (output.upgrade().unwrap(), state))
                .unwrap();
            let updated = compare(
                &current_output,
                current_state,
                output,
                element_state.as_ref().unwrap(),
            )
            .clone();
            *primary_scanout_output = Some((updated.downgrade(), element_state.unwrap()));
            return Some(updated);
        }

        primary_scanout_output
            .as_ref()
            .and_then(|(output, _)| output.upgrade())
    }

    pub fn current_output(&self) -> Option<Output> {
        self.0.as_ref().and_then(|(o, _)| o.upgrade())
    }
}

pub fn default_primary_scanout_output_compare<'a>(
    current_output: &'a Output,
    current_state: &RenderElementState,
    next_output: &'a Output,
    next_state: &RenderElementState,
) -> &'a Output {
    const VISIBLE_AREA_THRESHOLD: usize = 2;

    let current_mode = current_output.current_mode();
    let next_mode = next_output.current_mode();

    let next_mode_has_higher_refresh = next_mode
        .map(|next_mode| {
            current_mode
                .map(|current_mode| next_mode.refresh > current_mode.refresh)
                .unwrap_or(false)
        })
        .unwrap_or(false);

    let current_visible_area_threshold = current_state.visible_area * VISIBLE_AREA_THRESHOLD;
    let next_mode_visible_area_greater = next_state.visible_area >= current_visible_area_threshold;

    if next_mode_visible_area_greater || next_mode_has_higher_refresh {
        next_output
    } else {
        current_output
    }
}

#[derive(Default, Debug, Clone)]
pub struct RenderElementStates {
    pub states: HashMap<Id, RenderElementState>,
}

impl RenderElementStates {
    pub fn element_render_state(&self, id: impl Into<Id>) -> Option<RenderElementState> {
        self.states.get(&id.into()).copied()
    }

    pub fn element_was_presented(&self, id: impl Into<Id>) -> bool {
        self.element_render_state(id)
            .map(|state| state.presentation_state != RenderElementPresentationState::Skipped)
            .unwrap_or(false)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Kind {
    Cursor,
    #[default]
    Unspecified,
}

pub trait Element {
    fn id(&self) -> &Id;
    fn current_commit(&self) -> CommitCounter;
    fn location(&self, scale: Scale<f64>) -> Point<i32, Physical> {
        self.geometry(scale).loc
    }
    fn src(&self) -> Rectangle<f64, BufferCoords>;
    fn transform(&self) -> Transform {
        Transform::Normal
    }
    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical>;
    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        if commit != Some(self.current_commit()) {
            DamageSet::from_slice(&[Rectangle::from_size(self.geometry(scale).size)])
        } else {
            DamageSet::default()
        }
    }
    fn opaque_regions(&self, _scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        OpaqueRegions::default()
    }
    fn alpha(&self) -> f32 {
        1.0
    }
    fn kind(&self) -> Kind {
        Kind::default()
    }
}

pub trait RenderElement<R: Renderer>: Element {
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error>;

    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        let _ = renderer;
        None
    }
}

pub trait AsRenderElements<R>
where
    R: Renderer,
{
    type RenderElement: RenderElement<R>;
    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: Point<i32, Physical>,
        scale: Scale<f64>,
        alpha: f32,
    ) -> Vec<C>;
}

impl<E> Element for &E
where
    E: Element,
{
    fn id(&self) -> &Id {
        (*self).id()
    }

    fn current_commit(&self) -> CommitCounter {
        (*self).current_commit()
    }

    fn location(&self, scale: Scale<f64>) -> Point<i32, Physical> {
        (*self).location(scale)
    }

    fn src(&self) -> Rectangle<f64, BufferCoords> {
        (*self).src()
    }

    fn transform(&self) -> Transform {
        (*self).transform()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        (*self).geometry(scale)
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        (*self).damage_since(scale, commit)
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        (*self).opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        (*self).alpha()
    }

    fn kind(&self) -> Kind {
        (*self).kind()
    }
}

impl<R, E> RenderElement<R> for &E
where
    R: Renderer,
    E: RenderElement<R> + Element,
{
    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        (*self).underlying_storage(renderer)
    }

    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        (*self).draw(frame, src, dst, damage, opaque_regions)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! render_elements_internal {
    (@enum $(#[$attr:meta])* $vis:vis $name:ident; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher(std::convert::Infallible),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident $($custom:ident)+; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$($custom),+> {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher(std::convert::Infallible),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident $lt:lifetime; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$lt> {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher(std::convert::Infallible),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident $lt:lifetime $($custom:ident)+; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$lt, $($custom),+> {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher(std::convert::Infallible),
        }
    };

    (@enum $(#[$attr:meta])* $vis:vis $name:ident<$renderer:ident>; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
        {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher((std::marker::PhantomData<$renderer>, std::convert::Infallible)),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident<$renderer:ident, $($custom:ident),+>; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer>,
            )+
        {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher((std::marker::PhantomData<$renderer>, std::convert::Infallible)),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident<$lt:lifetime, $renderer:ident>; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$lt, $renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
        {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher((std::marker::PhantomData<$renderer>, std::convert::Infallible)),
        }
    };
    (@enum $(#[$attr:meta])* $vis:vis $name:ident<$lt:lifetime, $renderer:ident, $($custom:ident),+>; $($(#[$meta:meta])* $body:ident=$field:ty$( as <$other_renderer:ty>)?),* $(,)?) => {
        $(#[$attr])*
        #[allow(clippy::large_enum_variant)]
        $vis enum $name<$lt, $renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer>,
            )+
        {
            $(
                $(
                    #[$meta]
                )*
                $body($field)
            ),*,
            #[doc(hidden)]
            _GenericCatcher((std::marker::PhantomData<$renderer>, std::convert::Infallible)),
        }
    };
    (@call $name:ident; $($x:ident),*) => {
        $crate::backend::renderer::element::Element::$name($($x),*)
    };
    (@call $renderer:ty; $name:ident; $($x:ident),*) => {
        $crate::backend::renderer::element::RenderElement::<$renderer>::$name($($x),*)
    };
    (@call $renderer:ty as $other:ty; draw; $x:ident, $renderer_ref:ident, $frame:ident, $($tail:ident),*) => {
        $crate::backend::renderer::element::RenderElement::<$other>::draw($x, $renderer_ref.as_mut(), $frame.as_mut(), $($tail),*).map_err(Into::into)
    };
    (@call $renderer:ty as $other:ty; $name:ident; $($x:ident),*) => {
        $crate::backend::renderer::element::RenderElement::<$other>::$name($($x),*)
    };
    (@body $($(#[$meta:meta])* $body:ident=$field:ty),* $(,)?) => {
        fn id(&self) -> &$crate::backend::renderer::element::Id {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call id; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn location(&self, scale: $crate::utils::Scale<f64>) -> $crate::utils::Point<i32, $crate::utils::Physical> {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call location; x, scale)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn src(&self) -> $crate::utils::Rectangle<f64, $crate::utils::Buffer> {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call src; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn transform(&self) -> $crate::utils::Transform {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call transform; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn geometry(&self, scale: $crate::utils::Scale<f64>) -> $crate::utils::Rectangle<i32, $crate::utils::Physical> {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call geometry; x, scale)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn current_commit(&self) -> $crate::backend::renderer::utils::CommitCounter {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call current_commit; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn damage_since(&self, scale: $crate::utils::Scale<f64>, commit: Option<$crate::backend::renderer::utils::CommitCounter>) -> $crate::backend::renderer::utils::DamageSet<i32, $crate::utils::Physical> {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call damage_since; x, scale, commit)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn opaque_regions(&self, scale: $crate::utils::Scale<f64>) -> $crate::backend::renderer::utils::OpaqueRegions<i32, $crate::utils::Physical> {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call opaque_regions; x, scale)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn alpha(&self) -> f32 {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call alpha; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        fn kind(&self) -> $crate::backend::renderer::element::Kind {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call kind; x)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }
    };
    (@draw <$renderer:ty>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        fn draw(
            &self,
            frame: &mut <$renderer as $crate::backend::renderer::RendererSuper>::Frame<'_, '_>,
            src: $crate::utils::Rectangle<f64, $crate::utils::Buffer>,
            dst: $crate::utils::Rectangle<i32, $crate::utils::Physical>,
            damage: &[$crate::utils::Rectangle<i32, $crate::utils::Physical>],
            opaque_regions: &[$crate::utils::Rectangle<i32, $crate::utils::Physical>],
        ) -> Result<(), <$renderer as $crate::backend::renderer::RendererSuper>::Error>
        where
        $(
            $(
                $renderer: std::convert::AsMut<$other_renderer>,
                <$renderer as $crate::backend::renderer::RendererSuper>::Frame: std::convert::AsMut<<$other_renderer as $crate::backend::renderer::RendererSuper>::Frame>,
                <$other_renderer as $crate::backend::renderer::RendererSuper>::Error: Into<<$renderer as $crate::backend::renderer::RendererSuper>::Error>,
            )*
        )*
        {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call $renderer $(as $other_renderer)?; draw; x, frame, src, dst, damage, opaque_regions)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        #[inline]
        fn underlying_storage(&self, renderer: &mut $renderer) -> Option<$crate::backend::renderer::element::UnderlyingStorage<'_>>
        {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call $renderer $(as $other_renderer)?; underlying_storage; x, renderer)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }
    };
    (@draw $renderer:ty; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        fn draw(
            &self,
            frame: &mut <$renderer as $crate::backend::renderer::RendererSuper>::Frame<'_, '_>,
            src: $crate::utils::Rectangle<f64, $crate::utils::Buffer>,
            dst: $crate::utils::Rectangle<i32, $crate::utils::Physical>,
            damage: &[$crate::utils::Rectangle<i32, $crate::utils::Physical>],
            opaque_regions: &[$crate::utils::Rectangle<i32, $crate::utils::Physical>],
        ) -> Result<(), <$renderer as $crate::backend::renderer::RendererSuper>::Error>
        {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call $renderer $(as $other_renderer)?; draw; x, frame, src, dst, damage, opaque_regions)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }

        #[inline]
        fn underlying_storage(&self, renderer: &mut $renderer) -> Option<$crate::backend::renderer::element::UnderlyingStorage<'_>>
        {
            match self {
                $(
                    #[allow(unused_doc_comments)]
                    $(
                        #[$meta]
                    )*
                    Self::$body(x) => $crate::render_elements_internal!(@call $renderer $(as $other_renderer)?; underlying_storage; x, renderer)
                ),*,
                Self::_GenericCatcher(_) => unreachable!(),
            }
        }
    };
    (@impl $name:ident<$renderer:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$renderer> $crate::backend::renderer::element::Element for $name<$renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$renderer> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw <$renderer>; $($tail)*);
        }
    };
    (@impl $name:ident<$lt:lifetime, $renderer:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$lt, $renderer> $crate::backend::renderer::element::Element for $name<$lt, $renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$lt, $renderer> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$lt, $renderer>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw <$renderer>; $($tail)*);
        }
    };
    (@impl $name:ident<$renderer:ident, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$renderer, $($custom),+> $crate::backend::renderer::element::Element for $name<$renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
            )+
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$renderer, $($custom),+> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
            )+
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw <$renderer>; $($tail)*);
        }
    };
    (@impl $name:ident<$lt:lifetime, $renderer:ident, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$lt, $renderer, $($custom),+> $crate::backend::renderer::element::Element for $name<$lt, $renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
            )+
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$lt, $renderer, $($custom),+> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$lt, $renderer, $($custom),+>
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
            $(
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
            )+
            $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw <$renderer>; $($tail)*);
        }
    };
    (@impl $name:ident; $renderer:ident; $($tail:tt)*) => {
        impl $crate::backend::renderer::element::Element for $name
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$renderer> $crate::backend::renderer::element::RenderElement<$renderer> for $name
        where
            $renderer: $crate::backend::renderer::Renderer,
            <$renderer as $crate::backend::renderer::RendererSuper>::TextureId: 'static,
        {
            $crate::render_elements_internal!(@draw <$renderer>; $($tail)*);
        }
    };

    (@impl $name:ident<=$renderer:ty>; $($tail:tt)*) => {
        impl $crate::backend::renderer::element::Element for $name
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl $crate::backend::renderer::element::RenderElement<$renderer> for $name
        {
            $crate::render_elements_internal!(@draw $renderer; $($tail)*);
        }
    };
    (@impl $name:ident<=$renderer:ty, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$($custom),+> $crate::backend::renderer::element::Element for $name<$($custom),+>
        where
        $(
            $custom: $crate::backend::renderer::element::Element,
        )+
        $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$($custom),+> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$($custom),+>
        where
        $(
            $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
        )+
        $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw $renderer; $($tail)*);
        }
    };

    (@impl $name:ident<=$renderer:ty, $lt:lifetime>; $($tail:tt)*) => {
        impl<$lt> $crate::backend::renderer::element::Element for $name<$lt>
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$lt> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$lt>
        {
            $crate::render_elements_internal!(@draw $renderer; $($tail)*);
        }
    };

    (@impl $name:ident<=$renderer:ty, $lt:lifetime, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        impl<$lt, $($custom),+> $crate::backend::renderer::element::Element for $name<$lt, $($custom),+>
        where
        $(
            $custom: $crate::backend::renderer::element::Element,
        )+
        $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@body $($tail)*);
        }
        impl<$lt, $($custom),+> $crate::backend::renderer::element::RenderElement<$renderer> for $name<$lt, $($custom),+>
        where
        $(
            $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
        )+
        $($($target: $bound $(+ $additional_bound)*),+)?
        {
            $crate::render_elements_internal!(@draw $renderer; $($tail)*);
        }
    };

    (@from $name:ident<$renderer:ident>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl<$renderer> From<$field> for $name<$renderer>
            where
                $renderer: $crate::backend::renderer::Renderer,
                $(
                    $($renderer: std::convert::AsMut<$other_renderer>,)?
                )*
            {
                #[inline]
                fn from(field: $field) -> $name<$renderer> {
                    $name::$body(field)
                }
            }
        )*
    };
    (@from $name:ident<$renderer:ident, $custom:ident>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl<$renderer, $custom> From<$field> for $name<$renderer, $custom>
            where
                $renderer: $crate::backend::renderer::Renderer,
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
                $(
                    $($renderer: std::convert::AsMut<$other_renderer>,)?
                )*
            {
                #[inline]
                fn from(field: $field) -> $name<$renderer, $custom> {
                    $name::$body(field)
                }
            }
        )*
    };
    (@from $name:ident<$lt:lifetime, $renderer:ident>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl<$lt, $renderer> From<$field> for $name<$lt, $renderer>
            where
                $renderer: $crate::backend::renderer::Renderer,
                $(
                    $($renderer: std::convert::AsMut<$other_renderer>,)?
                )*
            {
                #[inline]
                fn from(field: $field) -> $name<$lt, $renderer> {
                    $name::$body(field)
                }
            }
        )*
    };
    (@from $name:ident<$lt:lifetime, $renderer:ident, $custom:ident>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl<$lt, $renderer, $custom> From<$field> for $name<$lt, $renderer, $custom>
            where
                $renderer: $crate::backend::renderer::Renderer,
                $custom: $crate::backend::renderer::element::RenderElement<$renderer> + $crate::backend::renderer::element::Element,
                $(
                    $($renderer: std::convert::AsMut<$other_renderer>,)?
                )*
            {
                #[inline]
                fn from(field: $field) -> $name<$lt, $renderer, $custom> {
                    $name::$body(field)
                }
            }
        )*
    };
    (@from $name:ident; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl From<$field> for $name {
                #[inline]
                fn from(field: $field) -> $name {
                    $name::$body(field)
                }
            }
        )*
    };
    (@from $name:ident<$lt:lifetime>; $($(#[$meta:meta])* $body:ident=$field:ty $(as <$other_renderer:ty>)?),* $(,)?) => {
        $(
            $(
                #[$meta]
            )*
            impl<$lt> From<$field> for $name<$lt> {
                #[inline]
                fn from(field: $field) -> $name<$lt> {
                    $name::$body(field)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! render_elements {
    ($(#[$attr:meta])* $vis:vis $name:ident<=$lt:lifetime, $renderer:ty, $custom:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name $lt $custom; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer, $lt, $custom> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$lt, $custom>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<=$lt:lifetime, $renderer:ty, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name $lt $($custom)+; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer, $lt, $($custom)+> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<=$lt:lifetime, $renderer:ty>; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name $lt; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer, $lt>; $($tail)*);
        $crate::render_elements_internal!(@from $name<$lt>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<=$renderer:ty, $custom:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name $custom; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer, $custom> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$custom>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<=$renderer:ty, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name $custom1 $custom2; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer, $($custom),+> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<=$renderer:ty>; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name; $($tail)*);
        $crate::render_elements_internal!(@impl $name<=$renderer>; $($tail)*);
        $crate::render_elements_internal!(@from $name; $($tail)*);
    };

    ($(#[$attr:meta])* $vis:vis $name:ident<$lt:lifetime, $renderer:ident, $custom:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$lt, $renderer, $custom>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$lt, $renderer, $custom> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$lt, $renderer, $custom>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<$lt:lifetime, $renderer:ident, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$lt, $renderer, $($custom),+>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$lt, $renderer, $($custom),+> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<$lt:lifetime, $renderer:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$lt, $renderer>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$lt, $renderer> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$lt, $renderer>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<$renderer:ident, $custom:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$renderer, $custom>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$renderer, $custom> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$renderer, $custom>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<$renderer:ident, $($custom:ident),+> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$renderer, $($custom),+>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$renderer, $($custom),+> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident<$renderer:ident> $(where $($target:ty: $bound:tt $(+ $additional_bound:tt)*),+)?; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name<$renderer>; $($tail)*);
        $crate::render_elements_internal!(@impl $name<$renderer> $(where $($target: $bound $(+ $additional_bound)*),+)?; $($tail)*);
        $crate::render_elements_internal!(@from $name<$renderer>; $($tail)*);
    };
    ($(#[$attr:meta])* $vis:vis $name:ident; $($tail:tt)*) => {
        $crate::render_elements_internal!(@enum $(#[$attr])* $vis $name; $($tail)*);
        $crate::render_elements_internal!(@impl $name; R; $($tail)*);
        $crate::render_elements_internal!(@from $name; $($tail)*);
    };
}

pub use render_elements;

#[derive(Debug)]
pub struct Wrap<C>(C);

impl<C> From<C> for Wrap<C> {
    fn from(from: C) -> Self {
        Self(from)
    }
}

impl<C> Element for Wrap<C>
where
    C: Element,
{
    fn id(&self) -> &Id {
        self.0.id()
    }

    fn current_commit(&self) -> CommitCounter {
        self.0.current_commit()
    }

    fn location(&self, scale: Scale<f64>) -> Point<i32, Physical> {
        self.0.location(scale)
    }

    fn src(&self) -> Rectangle<f64, BufferCoords> {
        self.0.src()
    }

    fn transform(&self) -> Transform {
        self.0.transform()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        self.0.geometry(scale)
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        self.0.damage_since(scale, commit)
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        self.0.opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        self.0.alpha()
    }

    fn kind(&self) -> Kind {
        self.0.kind()
    }
}

impl<R, C> RenderElement<R> for Wrap<C>
where
    R: Renderer,
    C: RenderElement<R>,
{
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        self.0.draw(frame, src, dst, damage, opaque_regions)
    }

    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.0.underlying_storage(renderer)
    }
}

#[cfg(all(test, feature = "renderer_gl"))]
#[allow(dead_code)]
mod tests;
