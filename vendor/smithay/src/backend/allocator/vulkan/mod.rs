//! Module for Buffers created using Vulkan.

#![forbid(unsafe_op_in_unsafe_fn)]

pub mod format;

use std::{
    ffi::CStr,
    fmt,
    os::unix::io::{FromRawFd, OwnedFd},
    sync::{mpsc, Arc, Weak},
};

use ash::{ext, khr, vk};
use bitflags::bitflags;
use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
use tracing::instrument;

#[cfg(feature = "backend_drm")]
use crate::backend::drm::DrmNode;
use crate::{
    backend::{
        allocator::dmabuf::DmabufFlags,
        vulkan::{version::Version, PhysicalDevice},
    },
    utils::{Buffer as BufferCoord, Size},
};

use super::{
    dmabuf::{AsDmabuf, Dmabuf, MAX_PLANES},
    Allocator, Buffer,
};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlags: vk::Flags {
        const TRANSFER_SRC = vk::ImageUsageFlags::TRANSFER_SRC.as_raw();

        const TRANSFER_DST = vk::ImageUsageFlags::TRANSFER_DST.as_raw();

        const SAMPLED = vk::ImageUsageFlags::SAMPLED.as_raw();

        const COLOR_ATTACHMENT = vk::ImageUsageFlags::COLOR_ATTACHMENT.as_raw();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not create allocator")]
    Setup,

    #[error("invalid buffer size")]
    InvalidSize,

    #[error("format is not supported")]
    UnsupportedFormat,

    #[error(transparent)]
    Vk(#[from] vk::Result),
}

pub struct VulkanAllocator {
    formats: Vec<FormatEntry>,
    images: Vec<ImageInner>,
    default_usage: ImageUsageFlags,
    remaining_allocations: u32,
    extension_fns: ExtensionFns,
    dropped_recv: mpsc::Receiver<ImageInner>,
    dropped_sender: mpsc::Sender<ImageInner>,
    phd: PhysicalDevice,
    #[cfg(feature = "backend_drm")]
    node: Option<DrmNode>,
    device: Arc<ash::Device>,
}

impl fmt::Debug for VulkanAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VulkanAllocator")
            .field("formats", &self.formats)
            .field("images", &self.images)
            .field("default_usage", &self.default_usage)
            .field("remaining_allocations", &self.remaining_allocations)
            .field("dropped_recv", &self.dropped_recv)
            .field("dropped_sender", &self.dropped_sender)
            .field("phd", &self.phd)
            .finish()
    }
}

impl VulkanAllocator {
    pub const MAX_INSTANCE_VERSION: Version = Version::VERSION_1_3;

    pub fn required_extensions(phd: &PhysicalDevice) -> Vec<&'static CStr> {
        let mut extensions = vec![
            ext::image_drm_format_modifier::NAME,
            ext::external_memory_dma_buf::NAME,
            khr::external_memory_fd::NAME,
        ];

        if phd.api_version() < Version::VERSION_1_2 {
            extensions.push(khr::image_format_list::NAME);
        }

        if phd.api_version() < Version::VERSION_1_3 {
            if phd.has_device_extension(ext::_4444_formats::NAME) {
                extensions.push(ext::_4444_formats::NAME);
            }
        }

        extensions
    }

    #[instrument(err, skip(phd), fields(physical_device = phd.name()))]
    pub fn new(phd: &PhysicalDevice, default_usage: ImageUsageFlags) -> Result<VulkanAllocator, Error> {
        if phd.instance().api_version() > Self::MAX_INSTANCE_VERSION {
            panic!("Exceeded maximum instance api version for VulkanAllocator (1.3 max)")
        }

        if default_usage.is_empty() {
            panic!("Default usage flags for allocator are empty")
        }

        let extensions = Self::required_extensions(phd);
        let extension_pointers = extensions.iter().copied().map(CStr::as_ptr).collect::<Vec<_>>();

        let queue_families = unsafe {
            phd.instance()
                .handle()
                .get_physical_device_queue_family_properties(phd.handle())
        };

        let queue_family_index = queue_families
            .iter()
            .position(|properties| properties.queue_flags.contains(vk::QueueFlags::TRANSFER))
            .or_else(|| {
                queue_families
                    .iter()
                    .position(|properties| properties.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            })
            .ok_or(Error::Setup)?;

        let queue_create_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&[0.0])];
        let create_info = vk::DeviceCreateInfo::default()
            .enabled_extension_names(&extension_pointers)
            .queue_create_infos(&queue_create_info);

        let instance = phd.instance().handle();
        let device = unsafe { instance.create_device(phd.handle(), &create_info, None) }?;

        let extension_fns = ExtensionFns {
            ext_image_format_modifier: ext::image_drm_format_modifier::Device::new(instance, &device),
            khr_external_memory_fd: khr::external_memory_fd::Device::new(instance, &device),
        };

        let (dropped_sender, dropped_recv) = mpsc::channel();

        #[cfg(feature = "backend_drm")]
        let node = phd
            .render_node()
            .ok()
            .flatten()
            .or_else(|| phd.primary_node().ok().flatten());

        let mut allocator = VulkanAllocator {
            formats: Vec::new(),
            images: Vec::new(),
            default_usage,
            remaining_allocations: phd.limits().max_memory_allocation_count,
            extension_fns,
            dropped_recv,
            dropped_sender,
            phd: phd.clone(),
            #[cfg(feature = "backend_drm")]
            node,
            device: Arc::new(device),
        };

        allocator.init_formats();

        Ok(allocator)
    }

    pub fn is_format_supported(&self, format: DrmFormat, usage: ImageUsageFlags) -> bool {
        if usage.is_empty() {
            return false;
        }

        unsafe { self.get_format_info(format, vk::ImageUsageFlags::from_raw(usage.bits())) }
            .ok()
            .is_some()
    }

    #[instrument(level = "trace", err)]
    #[profiling::function]
    pub fn create_buffer_with_usage(
        &mut self,
        width: u32,
        height: u32,
        fourcc: DrmFourcc,
        modifiers: &[DrmModifier],
        usage: ImageUsageFlags,
    ) -> Result<VulkanImage, Error> {
        self.cleanup();

        let vk_format = format::get_vk_format(fourcc).ok_or(Error::UnsupportedFormat)?;
        let vk_usage = vk::ImageUsageFlags::from_raw(usage.bits());

        if width == 0 || height == 0 {
            return Err(Error::InvalidSize);
        }

        if usage.is_empty() {
            return Err(Error::UnsupportedFormat);
        }

        if usage.contains(ImageUsageFlags::COLOR_ATTACHMENT) {
            let limits = self.phd.limits();

            if width > limits.max_framebuffer_width || height > limits.max_framebuffer_height {
                return Err(Error::InvalidSize);
            }
        }

        let modifiers = self.filter_modifiers(width, height, vk_usage, fourcc, modifiers);

        if modifiers.is_empty() {
            return Err(Error::UnsupportedFormat);
        }

        Ok(unsafe { self.create_image(width, height, vk_format, vk_usage, fourcc, &modifiers[..]) }?)
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.phd
    }
}

impl Allocator for VulkanAllocator {
    type Buffer = VulkanImage;
    type Error = Error;

    fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        fourcc: DrmFourcc,
        modifiers: &[DrmModifier],
    ) -> Result<VulkanImage, Self::Error> {
        self.create_buffer_with_usage(width, height, fourcc, modifiers, self.default_usage)
    }
}

impl Drop for VulkanAllocator {
    fn drop(&mut self) {
        unsafe {
            for image in &self.images {
                self.device.destroy_image(image.image, None);
                self.device.free_memory(image.memory, None);
            }

            self.device.destroy_device(None);
        }
    }
}

pub struct VulkanImage {
    inner: ImageInner,
    width: u32,
    height: u32,
    format: DrmFormat,
    #[cfg(feature = "backend_drm")]
    node: Option<DrmNode>,
    format_plane_count: u32,
    khr_external_memory_fd: khr::external_memory_fd::Device,
    dropped_sender: mpsc::Sender<ImageInner>,
    device: Weak<ash::Device>,
}

impl fmt::Debug for VulkanImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VulkanImage")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("inner", &self.inner)
            .finish()
    }
}

impl Buffer for VulkanImage {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn size(&self) -> Size<i32, BufferCoord> {
        (self.width as i32, self.height as i32).into()
    }

    fn format(&self) -> DrmFormat {
        self.format
    }
}

impl AsDmabuf for VulkanImage {
    type Error = ExportError;

    #[profiling::function]
    fn export(&self) -> Result<Dmabuf, Self::Error> {
        let device = self.device.upgrade().ok_or(ExportError::AllocatorDestroyed)?;

        if self.format_plane_count == 0 {
            return Err(ExportError::Failed);
        }

        assert!(
            self.format_plane_count as usize <= MAX_PLANES,
            "Vulkan implementation reported too many planes"
        );

        let create_info = vk::MemoryGetFdInfoKHR::default()
            .handle_type(vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT)
            .memory(self.inner.memory);

        let fd = unsafe { self.khr_external_memory_fd.get_memory_fd(&create_info) }?;
        // SAFETY: `vkGetMemoryFdKHR` creates a new file descriptor owned by the caller.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        let mut builder = Dmabuf::builder(
            self.size(),
            self.format().code,
            self.format().modifier,
            DmabufFlags::empty(),
        );

        for idx in 0..self.format_plane_count {
            let aspect_mask = match idx {
                0 => vk::ImageAspectFlags::MEMORY_PLANE_0_EXT,
                1 => vk::ImageAspectFlags::MEMORY_PLANE_1_EXT,
                2 => vk::ImageAspectFlags::MEMORY_PLANE_2_EXT,
                3 => vk::ImageAspectFlags::MEMORY_PLANE_3_EXT,
                _ => unreachable!(),
            };

            let subresource = vk::ImageSubresource::default().aspect_mask(aspect_mask);
            let layout = unsafe { device.get_image_subresource_layout(self.inner.image, subresource) };
            builder.add_plane(
                fd.try_clone().or(Err(ExportError::Failed))?,
                idx,
                layout.offset as u32,
                layout.row_pitch as u32,
            );
        }

        #[cfg(feature = "backend_drm")]
        if let Some(node) = self.node {
            builder.set_node(node);
        }

        Ok(builder.build().unwrap())
    }
}

impl Drop for VulkanImage {
    fn drop(&mut self) {
        let _ = self.dropped_sender.send(self.inner);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("allocator has been destroyed")]
    AllocatorDestroyed,

    #[error("could not export a dmabuf")]
    Failed,

    #[error(transparent)]
    Vk(#[from] vk::Result),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ImageInner {
    image: vk::Image,
    memory: vk::DeviceMemory,
}

#[derive(Debug)]
struct FormatEntry {
    format: DrmFormat,
    modifier_properties: vk::DrmFormatModifierPropertiesEXT,
}

struct ExtensionFns {
    ext_image_format_modifier: ext::image_drm_format_modifier::Device,
    khr_external_memory_fd: khr::external_memory_fd::Device,
}

impl VulkanAllocator {
    fn init_formats(&mut self) {
        for &fourcc in format::known_formats() {
            let vk_format = format::get_vk_format(fourcc).unwrap();
            let modifier_properties = self
                .phd
                .get_format_modifier_properties(vk_format)
                .expect("The Vulkan allocator requires VK_EXT_image_drm_format_modifier");

            for modifier_properties in modifier_properties {
                self.formats.push(FormatEntry {
                    format: DrmFormat {
                        code: fourcc,
                        modifier: DrmModifier::from(modifier_properties.drm_format_modifier),
                    },
                    modifier_properties,
                });
            }
        }
    }

    unsafe fn get_format_info(
        &self,
        format: DrmFormat,
        usage: vk::ImageUsageFlags,
    ) -> Result<Option<vk::ImageFormatProperties>, vk::Result> {
        let vk_format = format::get_vk_format(format.code);

        match vk_format {
            Some(vk_format) => {
                let mut image_drm_format_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
                    .drm_format_modifier(format.modifier.into())
                    .sharing_mode(vk::SharingMode::EXCLUSIVE);
                let format_info = vk::PhysicalDeviceImageFormatInfo2::default()
                    .format(vk_format)
                    .ty(vk::ImageType::TYPE_2D)
                    .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
                    .usage(usage)
                    .flags(vk::ImageCreateFlags::empty())
                    .push_next(&mut image_drm_format_info);
                let mut image_format_properties = vk::ImageFormatProperties2::default();

                let result = unsafe {
                    self.phd
                        .instance()
                        .handle()
                        .get_physical_device_image_format_properties2(
                            self.phd.handle(),
                            &format_info,
                            &mut image_format_properties,
                        )
                };

                result
                    .map(|_| Some(image_format_properties.image_format_properties))
                    .or_else(|result| {
                        if result == vk::Result::ERROR_FORMAT_NOT_SUPPORTED {
                            Ok(None)
                        } else {
                            Err(result)
                        }
                    })
            }

            None => Ok(None),
        }
    }

    fn filter_modifiers(
        &self,
        width: u32,
        height: u32,
        vk_usage: vk::ImageUsageFlags,
        fourcc: DrmFourcc,
        modifiers: &[DrmModifier],
    ) -> Vec<u64> {
        modifiers
            .iter()
            .copied()
            .filter_map(move |modifier| {
                let info = unsafe {
                    self.get_format_info(
                        DrmFormat {
                            code: fourcc,
                            modifier,
                        },
                        vk_usage,
                    )
                }
                .ok()
                .flatten()?;

                Some((modifier, info))
            })
            .filter(move |(_, properties)| {
                let max_extent = properties.max_extent;

                max_extent.width >= width
                && max_extent.height >= height
                && max_extent.depth >= 1
                && properties.sample_counts.contains(vk::SampleCountFlags::TYPE_1)
            })
            .map(|(modifier, _)| modifier)
            .map(Into::<u64>::into)
            .collect::<Vec<_>>()
    }

    unsafe fn create_image(
        &mut self,
        width: u32,
        height: u32,
        vk_format: vk::Format,
        vk_usage: vk::ImageUsageFlags,
        fourcc: DrmFourcc,
        modifiers: &[u64],
    ) -> Result<VulkanImage, vk::Result> {
        assert!(width > 0);
        assert!(height > 0);

        if self.remaining_allocations == 0 {
            todo!()
        }

        let mut modifier_list =
            vk::ImageDrmFormatModifierListCreateInfoEXT::default().drm_format_modifiers(modifiers);
        let mut external_memory_image_create_info = vk::ExternalMemoryImageCreateInfo::default()
            .handle_types(vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT);

        let image_create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk_format)
            .extent(vk::Extent3D {
                width,
                height,
                depth: 1,
            })
            .samples(vk::SampleCountFlags::TYPE_1)
            .mip_levels(1)
            .array_layers(1)
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(vk_usage)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .push_next(&mut modifier_list)
            .push_next(&mut external_memory_image_create_info);

        let mut guard = scopeguard::guard(
            ImageInner {
                image: unsafe { self.device.create_image(&image_create_info, None) }?,
                memory: vk::DeviceMemory::null(),
            },
            |inner| unsafe { self.device.destroy_image(inner.image, None) },
        );

        let format = {
            let mut image_modifier_properties = vk::ImageDrmFormatModifierPropertiesEXT::default();

            unsafe {
                self.extension_fns
                    .ext_image_format_modifier
                    .get_image_drm_format_modifier_properties(guard.image, &mut image_modifier_properties)
            }?;

            DrmFormat {
                code: fourcc,
                modifier: DrmModifier::from(image_modifier_properties.drm_format_modifier),
            }
        };

        let format_plane_count = self
            .formats
            .iter()
            .find(|entry| entry.format == format)
            .unwrap()
            .modifier_properties
            .drm_format_modifier_plane_count;

        let memory_reqs = unsafe { self.device.get_image_memory_requirements(guard.image) };
        let mut export_memory_allocate_info = vk::ExportMemoryAllocateInfo::default()
            .handle_types(vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT);
        let alloc_create_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_reqs.size)
            .push_next(&mut export_memory_allocate_info);

        unsafe {
            guard.memory = self.device.allocate_memory(&alloc_create_info, None)?;
            self.device.bind_image_memory(guard.image, guard.memory, 0)?;
        }

        let inner = scopeguard::ScopeGuard::into_inner(guard);

        self.images.push(inner);

        self.remaining_allocations -= 1;

        Ok(VulkanImage {
            inner,
            width,
            height,
            format,
            format_plane_count,
            khr_external_memory_fd: self.extension_fns.khr_external_memory_fd.clone(),
            dropped_sender: self.dropped_sender.clone(),
            device: Arc::downgrade(&self.device),
            #[cfg(feature = "backend_drm")]
            node: self.node,
        })
    }

    fn cleanup(&mut self) {
        let dropped = self.dropped_recv.try_iter().collect::<Vec<_>>();

        self.images.retain(|image| {
            let drop = dropped.contains(image);

            if drop {
                unsafe {
                    self.device.destroy_image(image.image, None);
                    self.device.free_memory(image.memory, None);
                }

                self.remaining_allocations = self
                    .remaining_allocations
                    .checked_add(1)
                    .expect("Remaining allocations overflowed");
                debug_assert!(
                    self.phd.limits().max_memory_allocation_count >= self.remaining_allocations,
                    "Too many allocations released",
                );
            }

            !drop
        })
    }
}
