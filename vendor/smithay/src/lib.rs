#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::upper_case_acronyms)]

pub mod backend;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod input;
pub mod output;
pub mod utils;
#[cfg(feature = "wayland_frontend")]
pub mod wayland;

#[cfg(feature = "xwayland")]
pub mod xwayland;

pub mod reexports;
