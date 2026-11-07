//! Format info tables for DRM formats.

use std::sync::Arc;

use super::Format;
use indexmap::IndexSet;

macro_rules! format_tables {
    (
        $($fourcc: ident {
            $(opaque: $opaque: ident,)?
            alpha: $alpha: expr,
            bpp: $bpp: expr,
            depth: $depth: expr $(,)?
        }),*
    ) => {
        pub const fn get_opaque(
            fourcc: $crate::backend::allocator::Fourcc,
        ) -> Option<$crate::backend::allocator::Fourcc> {
            match fourcc {
                $($(
                    $crate::backend::allocator::Fourcc::$fourcc
                        => Some($crate::backend::allocator::Fourcc::$opaque),
                )?)*
                _ => None,
            }
        }

        pub const fn get_transparent(
            fourcc: $crate::backend::allocator::Fourcc,
        ) -> Option<$crate::backend::allocator::Fourcc> {
            match fourcc {
                $($(
                    $crate::backend::allocator::Fourcc::$opaque
                        => Some($crate::backend::allocator::Fourcc::$fourcc),
                )?)*
                _ => None,
            }
        }

        pub const fn has_alpha(fourcc: $crate::backend::allocator::Fourcc) -> bool {
            match fourcc {
                $(
                    $crate::backend::allocator::Fourcc::$fourcc => $alpha,
                )*
                _ => false,
            }
        }

        pub const fn get_bpp(
            fourcc: $crate::backend::allocator::Fourcc,
        ) -> Option<usize> {
            match fourcc {
                $($crate::backend::allocator::Fourcc::$fourcc => Some($bpp),)*
                _ => None,
            }
        }

        pub const fn get_depth(
            fourcc: $crate::backend::allocator::Fourcc,
        ) -> Option<usize> {
            match fourcc {
                $($crate::backend::allocator::Fourcc::$fourcc => Some($depth),)*
                _ => None,
            }
        }

        fn _impl_formats() -> &'static [$crate::backend::allocator::Fourcc] {
            &[
                $(
                    $crate::backend::allocator::Fourcc::$fourcc,
                )*
            ]
        }
    };
}

format_tables! {
    R8 { alpha: false, bpp: 8, depth: 8 },

    R16 { alpha: false, bpp: 16, depth: 16 },

    Rg88 { alpha: false, bpp: 16, depth: 16 },

    Gr88 { alpha: false, bpp: 16, depth: 16 },

    Rg1616 { alpha: false, bpp: 32, depth: 32 },

    Gr1616 { alpha: false, bpp: 32, depth: 32 },

    Rgb332 { alpha: false, bpp: 8, depth: 8 },

    Bgr233 { alpha: false, bpp: 8, depth: 8 },

    Argb4444 {
        opaque: Xrgb4444,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Xrgb4444 { alpha: false, bpp: 16, depth: 12 },

    Abgr4444 {
        opaque: Xbgr4444,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Xbgr4444 { alpha: false, bpp: 16, depth: 12 },

    Rgba4444 {
        opaque: Rgbx4444,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Rgbx4444 { alpha: false, bpp: 16, depth: 12 },

    Bgra4444 {
        opaque: Bgrx4444,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Bgrx4444 { alpha: false, bpp: 16, depth: 12 },

    Argb1555 {
        opaque: Xrgb1555,
        alpha: true,
        bpp: 16,
        depth: 16
    },

    Xrgb1555 { alpha: false, bpp: 16, depth: 15 },

    Abgr1555 {
        opaque: Xbgr1555,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Xbgr1555 { alpha: false, bpp: 16, depth: 15 },

    Rgba5551 {
        opaque: Rgbx5551,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Rgbx5551 { alpha: false, bpp: 16, depth: 15 },

    Bgra5551 {
        opaque: Bgrx5551,
        alpha: true,
        bpp: 16,
        depth: 16,
    },

    Bgrx5551 { alpha: false, bpp: 16, depth: 15 },

    Rgb565 { alpha: false, bpp: 16, depth: 16 },

    Bgr565 { alpha: false, bpp: 16, depth: 16 },

    Rgb888 { alpha: false, bpp: 24, depth: 24 },

    Bgr888 { alpha: false, bpp: 24, depth: 24 },

    Argb8888 {
        opaque: Xrgb8888,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Xrgb8888 { alpha: false, bpp: 32, depth: 24 },

    Abgr8888 {
        opaque: Xbgr8888,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Xbgr8888 { alpha: false, bpp: 32, depth: 24 },

    Rgba8888 {
        opaque: Rgbx8888,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Rgbx8888 { alpha: false, bpp: 32, depth: 24 },

    Bgra8888 {
        opaque: Bgrx8888,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Bgrx8888 { alpha: false, bpp: 32, depth: 24 },

    Argb2101010 {
        opaque: Xrgb2101010,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Xrgb2101010 { alpha: false, bpp: 32, depth: 30 },

    Abgr2101010 {
        opaque: Xbgr2101010,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Xbgr2101010 { alpha: false, bpp: 32, depth: 30 },

    Rgba1010102 {
        opaque: Rgbx1010102,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Rgbx1010102 { alpha: false, bpp: 32, depth: 30 },

    Bgra1010102 {
        opaque: Bgrx1010102,
        alpha: true,
        bpp: 32,
        depth: 32,
    },

    Bgrx1010102 { alpha: false, bpp: 32, depth: 30 },

    Argb16161616f {
        opaque: Xrgb16161616f,
        alpha: true,
        bpp: 64,
        depth: 64
    },

    Xrgb16161616f { alpha: false, bpp: 64, depth: 48 },

    Abgr16161616f {
        opaque: Xbgr16161616f,
        alpha: true,
        bpp: 64,
        depth: 64,
    },

    Xbgr16161616f { alpha: false, bpp: 64, depth: 48 },

    Axbxgxrx106106106106 { alpha: true, bpp: 64, depth: 40 }

}

#[derive(Debug, Default, Clone)]
pub struct FormatSet {
    formats: Arc<IndexSet<Format>>,
}

impl FormatSet {
    #[cfg(any(feature = "backend_egl", feature = "backend_drm"))]
    pub(crate) fn from_formats(formats: IndexSet<Format>) -> Self {
        FormatSet {
            formats: Arc::new(formats),
        }
    }
}

impl FormatSet {
    pub fn iter(&self) -> FormatSetIter<'_> {
        FormatSetIter {
            inner: self.formats.iter(),
        }
    }

    pub fn contains(&self, format: &Format) -> bool {
        self.formats.contains(format)
    }

    pub fn intersection<'a>(&'a self, other: &'a FormatSet) -> FormatSetIntersection<'a> {
        FormatSetIntersection {
            inner: self.formats.intersection(&other.formats),
        }
    }

    pub fn indexset(&self) -> &IndexSet<Format> {
        &self.formats
    }
}

#[derive(Debug)]
pub struct FormatSetIntersection<'a> {
    inner: indexmap::set::Intersection<'a, Format, std::collections::hash_map::RandomState>,
}

impl<'a> Iterator for FormatSetIntersection<'a> {
    type Item = &'a Format;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl IntoIterator for FormatSet {
    type Item = Format;

    type IntoIter = FormatSetIntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        FormatSetIntoIter {
            inner: (*self.formats).clone().into_iter(),
        }
    }
}

impl FromIterator<Format> for FormatSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Format>>(iter: T) -> Self {
        Self {
            formats: Arc::new(IndexSet::from_iter(iter)),
        }
    }
}

#[derive(Debug)]
pub struct FormatSetIter<'a> {
    inner: indexmap::set::Iter<'a, Format>,
}

impl<'a> Iterator for FormatSetIter<'a> {
    type Item = &'a Format;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[derive(Debug)]
pub struct FormatSetIntoIter {
    inner: indexmap::set::IntoIter<Format>,
}

impl Iterator for FormatSetIntoIter {
    type Item = Format;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::{_impl_formats, get_bpp, get_depth, get_opaque, get_transparent, has_alpha};

    #[test]
    fn opaque_neq() {
        for &format in _impl_formats() {
            if let Some(opaque) = get_opaque(format) {
                assert_ne!(
                    format, opaque,
                    "{}'s opaque alternative is the same format",
                    format
                );
            }
        }
    }

    #[test]
    fn opaque_inverse() {
        for &format in _impl_formats() {
            if let Some(opaque) = get_opaque(format) {
                let transparent = get_transparent(opaque);
                assert_eq!(
                    Some(format),
                    transparent,
                    "{}'s opaque alternative {} doesn't cleanly convert back, got: {:?}",
                    format,
                    opaque,
                    transparent
                );
            }
        }
    }

    #[test]
    fn opaque_alternatives() {
        for &format in _impl_formats() {
            if let Some(opaque) = get_opaque(format) {
                let result = get_opaque(opaque);

                assert!(
                    result.is_none(),
                    "Format {format} has an opaque alternative, {opaque}. However {opaque} reports an opaque alternative {opaque_opaque:?} which is incorrect",
                    format = format,
                    opaque = opaque,
                    opaque_opaque = result,
                );
            }
        }
    }

    #[test]
    fn opaque_has_same_bpp() {
        for &format in _impl_formats() {
            if let Some(opaque) = get_opaque(format) {
                let format_bpp = get_bpp(format);
                let opaque_bpp = get_bpp(opaque);

                assert_eq!(
                    format_bpp,
                    opaque_bpp,
                    "Format {format} has a bpp of {format_bpp:?}. However the opaque alternative {opaque} has a different bpp of {opaque_bpp:?}",
                    format_bpp = get_bpp(format),
                    opaque = opaque,
                    opaque_bpp = get_bpp(opaque),
                );
            }
        }
    }

    #[test]
    fn format_with_opaque_has_alpha() {
        for &format in _impl_formats() {
            if let Some(opaque) = get_opaque(format) {
                assert!(
                    has_alpha(format),
                    "{} has an opaque alternative but does not state it has an alpha component",
                    format
                );

                assert!(
                    !has_alpha(opaque),
                    "opaque alternative to {} ({}) has an alpha channel",
                    format,
                    opaque
                );
            }
        }
    }

    #[test]
    fn format_bpp_greater_or_equal_than_depth() {
        for &format in _impl_formats() {
            assert!(
                get_depth(format) <= get_bpp(format),
                "{} has a depth higher than its bpp",
                format
            );
        }
    }
}
