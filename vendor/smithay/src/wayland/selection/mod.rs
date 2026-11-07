//! Handler utilities for the various selection protocols.

use std::os::unix::io::OwnedFd;

use crate::input::{Seat, SeatHandler};

pub mod data_device;
pub mod ext_data_control;
pub mod primary_selection;
pub mod wlr_data_control;

mod device;
mod offer;
mod seat_data;
mod source;

pub use source::SelectionSource;

pub trait SelectionHandler: Sized + SeatHandler {
    type SelectionUserData: Clone + Send + Sync + 'static;

    #[allow(unused_variables)]
    fn new_selection(&mut self, ty: SelectionTarget, source: Option<SelectionSource>, seat: Seat<Self>) {}

    #[allow(unused_variables)]
    fn send_selection(
        &mut self,
        ty: SelectionTarget,
        mime_type: String,
        fd: OwnedFd,
        seat: Seat<Self>,
        user_data: &Self::SelectionUserData,
    ) {
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionTarget {
    Primary,
    Clipboard,
}

pub(crate) mod private {
    macro_rules! selection_dispatch {
        ($what:ident; $enum:ident ( $($c1:tt)* ) => $x:expr) => {
            match $what {
                $enum::DataDevice($($c1)*) => $x,
                $enum::Primary($($c1)*) => $x,
                $enum::WlrDataControl($($c1)*) => $x,
                $enum::ExtDataControl($($c1)*) => $x,
            }
        };
        ($what:ident$(, $what_next:ident)+; $enum:ident ( $($c1:tt)*) $(, $enum_next:ident ( $($c2:tt)* ) )+ => $x:expr) => {
            match ($what$(, $what_next)*) {
                ($enum::DataDevice($($c1)*)$(, $enum_next::DataDevice($($c2)*))*) => $x,
                ($enum::Primary($($c1)*)$(, $enum_next::Primary($($c2)*))*) => $x,
                ($enum::WlrDataControl($($c1)*)$(, $enum_next::WlrDataControl($($c2)*))*) => $x,
                ($enum::ExtDataControl($($c1)*)$(, $enum_next::ExtDataControl($($c2)*))*) => $x,
                _ => unreachable!(),
            }
        };
    }

    pub(crate) use selection_dispatch;
}
