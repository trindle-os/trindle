//! The [`Version`] type.

use std::{
    cmp::Ordering,
    fmt::{self, Formatter},
};

use ash::vk;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Version {
    pub variant: u32,

    pub major: u32,

    pub minor: u32,

    pub patch: u32,
}

impl Version {
    pub const VERSION_1_0: Version = Version::from_raw(vk::API_VERSION_1_0);

    pub const VERSION_1_1: Version = Version::from_raw(vk::API_VERSION_1_1);

    pub const VERSION_1_2: Version = Version::from_raw(vk::API_VERSION_1_2);

    pub const VERSION_1_3: Version = Version::from_raw(vk::API_VERSION_1_3);

    pub const SMITHAY: Version = Version {
        variant: 0,
        major: 0,
        minor: 3,
        patch: 0,
    };

    pub const fn from_raw(raw: u32) -> Version {
        Version {
            variant: vk::api_version_variant(raw),
            major: vk::api_version_major(raw),
            minor: vk::api_version_minor(raw),
            patch: vk::api_version_patch(raw),
        }
    }

    pub const fn to_raw(self) -> u32 {
        vk::make_api_version(self.variant, self.major, self.minor, self.patch)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{} variant {}",
            self.major, self.minor, self.patch, self.variant
        )
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.variant.cmp(&other.variant) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            ord => return ord,
        }

        self.patch.cmp(&other.patch)
    }
}
