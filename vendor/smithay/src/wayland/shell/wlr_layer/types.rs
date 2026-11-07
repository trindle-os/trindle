use std::cmp::Ordering;

use wayland_protocols_wlr::layer_shell::v1::server::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};
use wayland_server::WEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

impl TryFrom<WEnum<zwlr_layer_shell_v1::Layer>> for Layer {
    type Error = (zwlr_layer_shell_v1::Error, String);

    #[inline]
    fn try_from(layer: WEnum<zwlr_layer_shell_v1::Layer>) -> Result<Self, Self::Error> {
        use zwlr_layer_shell_v1::Layer;

        match layer {
            WEnum::Value(Layer::Background) => Ok(Self::Background),
            WEnum::Value(Layer::Bottom) => Ok(Self::Bottom),
            WEnum::Value(Layer::Top) => Ok(Self::Top),
            WEnum::Value(Layer::Overlay) => Ok(Self::Overlay),
            layer => Err((
                zwlr_layer_shell_v1::Error::InvalidLayer,
                format!("invalid layer: {:?}", layer),
            )),
        }
    }
}

impl Default for Layer {
    #[inline]
    fn default() -> Self {
        Self::Background
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardInteractivity {
    None,
    Exclusive,
    OnDemand,
}

impl Default for KeyboardInteractivity {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<WEnum<zwlr_layer_surface_v1::KeyboardInteractivity>> for KeyboardInteractivity {
    type Error = (zwlr_layer_surface_v1::Error, String);

    #[inline]
    fn try_from(ki: WEnum<zwlr_layer_surface_v1::KeyboardInteractivity>) -> Result<Self, Self::Error> {
        use zwlr_layer_surface_v1::KeyboardInteractivity;

        match ki {
            WEnum::Value(KeyboardInteractivity::None) => Ok(Self::None),
            WEnum::Value(KeyboardInteractivity::Exclusive) => Ok(Self::Exclusive),
            WEnum::Value(KeyboardInteractivity::OnDemand) => Ok(Self::OnDemand),
            ki => Err((
                zwlr_layer_surface_v1::Error::InvalidKeyboardInteractivity,
                format!("wrong keyboard interactivity value: {:?}", ki),
            )),
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Anchor: u32 {
        const TOP = 1;
        const BOTTOM = 2;
        const LEFT = 4;
        const RIGHT = 8;
    }
}

impl Anchor {
    pub fn anchored_horizontally(&self) -> bool {
        self.contains(Self::LEFT) && self.contains(Self::RIGHT)
    }
    pub fn anchored_vertically(&self) -> bool {
        self.contains(Self::TOP) && self.contains(Self::BOTTOM)
    }
}

impl Default for Anchor {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl TryFrom<WEnum<zwlr_layer_surface_v1::Anchor>> for Anchor {
    type Error = (zwlr_layer_surface_v1::Error, String);

    #[inline]
    fn try_from(anchor: WEnum<zwlr_layer_surface_v1::Anchor>) -> Result<Self, Self::Error> {
        let a = if let WEnum::Value(anchor) = anchor {
            Anchor::from_bits(anchor.bits())
        } else {
            None
        };

        a.ok_or((
            zwlr_layer_surface_v1::Error::InvalidAnchor,
            format!("invalid anchor {:?}", anchor),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExclusiveZone {
    Exclusive(u32),
    Neutral,
    DontCare,
}

impl Default for ExclusiveZone {
    #[inline]
    fn default() -> Self {
        Self::Neutral
    }
}

impl From<i32> for ExclusiveZone {
    #[inline]
    fn from(v: i32) -> Self {
        match v.cmp(&0) {
            Ordering::Greater => Self::Exclusive(v as u32),
            Ordering::Equal => Self::Neutral,
            Ordering::Less => Self::DontCare,
        }
    }
}

impl From<ExclusiveZone> for i32 {
    #[inline]
    fn from(z: ExclusiveZone) -> i32 {
        match z {
            ExclusiveZone::Exclusive(v) => v as i32,
            ExclusiveZone::Neutral => 0,
            ExclusiveZone::DontCare => -1,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Margins {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}
