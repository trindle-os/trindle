use std::collections::HashSet;

use crate::{
    backend::{
        allocator::{
            dmabuf::{AsDmabuf, Dmabuf},
            Buffer, Slot,
        },
        drm::Framebuffer,
        renderer::{
            damage::OutputDamageTracker,
            element::{Element, Id, RenderElement, RenderElementStates},
            sync::SyncPoint,
            utils::{CommitCounter, DamageSet, DamageSnapshot, OpaqueRegions},
            Bind, Blit, Color32F, Frame, Renderer,
        },
    },
    output::OutputNoMode,
    utils::{Buffer as BufferCoords, Physical, Point, Rectangle, Scale, Size, Transform},
};

use super::{DrmScanoutBuffer, ScanoutBuffer};

pub struct RenderFrameResult<'a, B: Buffer, F: Framebuffer, E> {
    pub is_empty: bool,
    pub states: RenderElementStates,
    pub primary_element: PrimaryPlaneElement<'a, B, F, E>,
    pub overlay_elements: Vec<&'a E>,
    pub cursor_element: Option<&'a E>,

    pub(super) primary_plane_element_id: Id,
    pub(super) supports_fencing: bool,
}

impl<B: Buffer, F: Framebuffer, E> RenderFrameResult<'_, B, F, E> {
    pub fn needs_sync(&self) -> bool {
        if let PrimaryPlaneElement::Swapchain(ref element) = self.primary_element {
            !self.supports_fencing || !element.sync.is_exportable()
        } else {
            false
        }
    }
}

struct SwapchainElement<'a, 'b, B: Buffer> {
    id: Id,
    slot: &'a Slot<B>,
    transform: Transform,
    damage: &'b DamageSnapshot<i32, BufferCoords>,
}

impl<B: Buffer> Element for SwapchainElement<'_, '_, B> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.damage.current_commit()
    }

    fn src(&self) -> Rectangle<f64, BufferCoords> {
        Rectangle::from_size(self.slot.size()).to_f64()
    }

    fn geometry(&self, _scale: Scale<f64>) -> Rectangle<i32, Physical> {
        Rectangle::from_size(self.slot.size().to_logical(1, self.transform).to_physical(1))
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        self.damage
            .damage_since(commit)
            .map(|d| {
                d.into_iter()
                    .map(|d| d.to_logical(1, self.transform, &self.slot.size()).to_physical(1))
                    .collect()
            })
            .unwrap_or_else(|| DamageSet::from_slice(&[self.geometry(scale)]))
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        OpaqueRegions::from_slice(&[self.geometry(scale)])
    }
}

enum FrameResultDamageElement<'a, 'b, E, B: Buffer> {
    Element(&'a E),
    Swapchain(SwapchainElement<'a, 'b, B>),
}

impl<E, B> Element for FrameResultDamageElement<'_, '_, E, B>
where
    E: Element,
    B: Buffer,
{
    fn id(&self) -> &Id {
        match self {
            FrameResultDamageElement::Element(e) => e.id(),
            FrameResultDamageElement::Swapchain(e) => e.id(),
        }
    }

    fn current_commit(&self) -> CommitCounter {
        match self {
            FrameResultDamageElement::Element(e) => e.current_commit(),
            FrameResultDamageElement::Swapchain(e) => e.current_commit(),
        }
    }

    fn src(&self) -> Rectangle<f64, BufferCoords> {
        match self {
            FrameResultDamageElement::Element(e) => e.src(),
            FrameResultDamageElement::Swapchain(e) => e.src(),
        }
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        match self {
            FrameResultDamageElement::Element(e) => e.geometry(scale),
            FrameResultDamageElement::Swapchain(e) => e.geometry(scale),
        }
    }

    fn location(&self, scale: Scale<f64>) -> Point<i32, Physical> {
        match self {
            FrameResultDamageElement::Element(e) => e.location(scale),
            FrameResultDamageElement::Swapchain(e) => e.location(scale),
        }
    }

    fn transform(&self) -> Transform {
        match self {
            FrameResultDamageElement::Element(e) => e.transform(),
            FrameResultDamageElement::Swapchain(e) => e.transform(),
        }
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        match self {
            FrameResultDamageElement::Element(e) => e.damage_since(scale, commit),
            FrameResultDamageElement::Swapchain(e) => e.damage_since(scale, commit),
        }
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        match self {
            FrameResultDamageElement::Element(e) => e.opaque_regions(scale),
            FrameResultDamageElement::Swapchain(e) => e.opaque_regions(scale),
        }
    }
}

#[derive(Debug)]
pub enum PrimaryPlaneElement<'a, B: Buffer, F: Framebuffer, E> {
    Swapchain(PrimarySwapchainElement<B, F>),
    Element(&'a E),
}

#[derive(Debug, thiserror::Error)]
pub enum BlitFrameResultError<R: std::error::Error, E: std::error::Error> {
    #[error(transparent)]
    Rendering(R),
    #[error(transparent)]
    Export(E),
}

impl<B, F, E> RenderFrameResult<'_, B, F, E>
where
    B: Buffer,
    F: Framebuffer,
{
    pub fn damage_from_age<'d>(
        &self,
        damage_tracker: &'d mut OutputDamageTracker,
        age: usize,
        filter: impl IntoIterator<Item = Id>,
    ) -> Result<(Option<&'d Vec<Rectangle<i32, Physical>>>, RenderElementStates), OutputNoMode>
    where
        E: Element,
    {
        #[allow(clippy::mutable_key_type)]
        let filter_ids: HashSet<Id> = filter.into_iter().collect();

        let mut elements: Vec<FrameResultDamageElement<'_, '_, E, B>> =
            Vec::with_capacity(usize::from(self.cursor_element.is_some()) + self.overlay_elements.len() + 1);
        if let Some(cursor) = self.cursor_element {
            if !filter_ids.contains(cursor.id()) {
                elements.push(FrameResultDamageElement::Element(cursor));
            }
        }

        elements.extend(
            self.overlay_elements
                .iter()
                .filter(|e| !filter_ids.contains(e.id()))
                .map(|e| FrameResultDamageElement::Element(*e)),
        );

        let primary_render_element = match &self.primary_element {
            PrimaryPlaneElement::Swapchain(PrimarySwapchainElement {
                slot,
                transform,
                damage,
                ..
            }) => FrameResultDamageElement::Swapchain(SwapchainElement {
                id: self.primary_plane_element_id.clone(),
                transform: *transform,
                slot: match &slot.buffer {
                    ScanoutBuffer::Swapchain(slot) => slot,
                    _ => unreachable!(),
                },
                damage,
            }),
            PrimaryPlaneElement::Element(e) => FrameResultDamageElement::Element(*e),
        };

        elements.push(primary_render_element);

        damage_tracker.damage_output(age, &elements)
    }
}

impl<'a, B, F, E> RenderFrameResult<'a, B, F, E>
where
    B: Buffer + AsDmabuf,
    <B as AsDmabuf>::Error: std::fmt::Debug,
    F: Framebuffer,
{
    #[allow(clippy::too_many_arguments)]
    pub fn blit_frame_result<R>(
        &self,
        size: impl Into<Size<i32, Physical>>,
        transform: Transform,
        scale: impl Into<Scale<f64>>,
        renderer: &mut R,
        framebuffer: &mut R::Framebuffer<'_>,
        damage: impl IntoIterator<Item = Rectangle<i32, Physical>>,
        filter: impl IntoIterator<Item = Id>,
    ) -> Result<SyncPoint, BlitFrameResultError<R::Error, <B as AsDmabuf>::Error>>
    where
        R: Renderer + Bind<Dmabuf> + Blit,
        R::TextureId: 'static,
        E: Element + RenderElement<R>,
    {
        let size = size.into();
        let scale = scale.into();
        #[allow(clippy::mutable_key_type)]
        let filter_ids: HashSet<Id> = filter.into_iter().collect();
        let damage = damage.into_iter().collect::<Vec<_>>();

        if damage.is_empty() {
            return Ok(SyncPoint::signaled());
        }

        let mut opaque_regions: Vec<Rectangle<i32, Physical>> = Vec::new();

        let mut elements_to_render: Vec<&'a E> =
            Vec::with_capacity(usize::from(self.cursor_element.is_some()) + self.overlay_elements.len() + 1);

        if let Some(cursor_element) = self.cursor_element.as_ref() {
            if !filter_ids.contains(cursor_element.id()) {
                elements_to_render.push(*cursor_element);
                opaque_regions.extend(cursor_element.opaque_regions(scale));
            }
        }

        for element in self
            .overlay_elements
            .iter()
            .filter(|e| !filter_ids.contains(e.id()))
        {
            elements_to_render.push(element);
            opaque_regions.extend(element.opaque_regions(scale));
        }

        let primary_dmabuf = match &self.primary_element {
            PrimaryPlaneElement::Swapchain(PrimarySwapchainElement { slot, sync, .. }) => {
                let dmabuf = match &slot.buffer {
                    ScanoutBuffer::Swapchain(slot) => slot.export().map_err(BlitFrameResultError::Export)?,
                    _ => unreachable!(),
                };
                let size = dmabuf.size();
                let geometry = Rectangle::from_size(size.to_logical(1, Transform::Normal).to_physical(1));
                opaque_regions.push(geometry);
                Some((sync.clone(), dmabuf, geometry))
            }
            PrimaryPlaneElement::Element(e) => {
                elements_to_render.push(*e);
                opaque_regions.extend(e.opaque_regions(scale));
                None
            }
        };

        let clear_damage =
            Rectangle::subtract_rects_many_in_place(damage.clone(), opaque_regions.iter().copied());

        let mut sync: Option<SyncPoint> = None;
        if !clear_damage.is_empty() {
            tracing::trace!("clearing frame damage {:#?}", clear_damage);

            let mut frame = renderer
                .render(framebuffer, size, transform)
                .map_err(BlitFrameResultError::Rendering)?;

            frame
                .clear(Color32F::BLACK, &clear_damage)
                .map_err(BlitFrameResultError::Rendering)?;

            sync = Some(frame.finish().map_err(BlitFrameResultError::Rendering)?);
        }

        if let Some((sync, mut dmabuf, geometry)) = primary_dmabuf {
            let blit_damage = damage
                .iter()
                .filter_map(|d| d.intersection(geometry))
                .collect::<Vec<_>>();

            tracing::trace!("blitting frame with damage: {:#?}", blit_damage);

            renderer.wait(&sync).map_err(BlitFrameResultError::Rendering)?;
            let fb = renderer
                .bind(&mut dmabuf)
                .map_err(BlitFrameResultError::Rendering)?;
            for rect in blit_damage {
                renderer
                    .blit(
                        &fb,
                        framebuffer,
                        rect,
                        rect,
                        crate::backend::renderer::TextureFilter::Linear,
                    )
                    .map_err(BlitFrameResultError::Rendering)?;
            }
        }

        if !elements_to_render.is_empty() {
            tracing::trace!("drawing {} frame element(s)", elements_to_render.len());

            let mut frame = renderer
                .render(framebuffer, size, transform)
                .map_err(BlitFrameResultError::Rendering)?;

            for element in elements_to_render.iter().rev() {
                let src = element.src();
                let dst = element.geometry(scale);
                let element_damage = damage
                    .iter()
                    .filter_map(|d| {
                        d.intersection(dst).map(|mut d| {
                            d.loc -= dst.loc;
                            d
                        })
                    })
                    .collect::<Vec<_>>();

                if element_damage.is_empty() {
                    continue;
                }

                tracing::trace!("drawing frame element with damage: {:#?}", element_damage);

                element
                    .draw(&mut frame, src, dst, &element_damage, &[])
                    .map_err(BlitFrameResultError::Rendering)?;
            }

            Ok(frame.finish().map_err(BlitFrameResultError::Rendering)?)
        } else {
            Ok(sync.unwrap_or_default())
        }
    }
}

impl<B: Buffer + std::fmt::Debug, F: Framebuffer + std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug
    for RenderFrameResult<'_, B, F, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderFrameResult")
            .field("is_empty", &self.is_empty)
            .field("states", &self.states)
            .field("primary_element", &self.primary_element)
            .field("overlay_elements", &self.overlay_elements)
            .field("cursor_element", &self.cursor_element)
            .finish()
    }
}

#[derive(Debug)]
pub struct PrimarySwapchainElement<B: Buffer, F: Framebuffer> {
    pub(super) slot: DrmScanoutBuffer<B, F>,
    pub sync: SyncPoint,
    pub transform: Transform,
    pub damage: DamageSnapshot<i32, BufferCoords>,
}

impl<B: Buffer, F: Framebuffer> PrimarySwapchainElement<B, F> {
    #[inline]
    pub fn buffer(&self) -> &B {
        match &self.slot.buffer {
            ScanoutBuffer::Swapchain(slot) => slot,
            _ => unreachable!(),
        }
    }
}
