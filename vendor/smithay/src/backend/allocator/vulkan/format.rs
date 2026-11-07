//! Format conversions between Vulkan and DRM formats.

macro_rules! vk_format_table {
    (
        $(
            $(#[$conv_meta:meta])*
            $fourcc: ident => $vk: ident
        ),* $(,)?
    ) => {
        pub const fn get_vk_format(fourcc: $crate::backend::allocator::Fourcc) -> Option<ash::vk::Format> {
            match fourcc {
                $(
                    $(#[$conv_meta])*
                    $crate::backend::allocator::Fourcc::$fourcc => Some(ash::vk::Format::$vk),
                )*

                _ => None,
            }
        }

        pub const fn known_formats() -> &'static [$crate::backend::allocator::Fourcc] {
            &[
                $(
                    $crate::backend::allocator::Fourcc::$fourcc
                ),*
            ]
        }
    };
}

vk_format_table! {
    Argb8888 => B8G8R8A8_SRGB,
    Xrgb8888 => B8G8R8A8_SRGB,

    Abgr8888 => R8G8B8A8_SRGB,
    Xbgr8888 => R8G8B8A8_SRGB,

    #[cfg(target_endian = "little")]
    Rgba8888 => A8B8G8R8_SRGB_PACK32,
    #[cfg(target_endian = "little")]
    Rgbx8888 => A8B8G8R8_SRGB_PACK32,

    #[cfg(target_endian = "little")]
    Argb2101010 => A2R10G10B10_UNORM_PACK32,
    #[cfg(target_endian = "little")]
    Xrgb2101010 => A2R10G10B10_UNORM_PACK32,

    #[cfg(target_endian = "little")]
    Abgr2101010 => A2B10G10R10_UNORM_PACK32,
    #[cfg(target_endian = "little")]
    Xbgr2101010 => A2B10G10R10_UNORM_PACK32,
}
