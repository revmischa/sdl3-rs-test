#![crate_name = "sdl3"]
#![crate_type = "lib"]

pub extern crate libc;

// #[macro_use]
// extern crate lazy_static;

pub extern crate sdl3_sys as sys;

pub use crate::sdl::*;

#[macro_use]
// mod macros;
// pub mod audio;
// pub mod controller;
pub mod event;
// pub mod filesystem;
// pub mod haptic;
// pub mod hint;
// pub mod joystick;
// pub mod keyboard;
// pub mod log;
// pub mod messagebox;
// pub mod mouse;
pub mod pixels;
pub mod rect;
pub mod render;
// pub mod rwops;
mod sdl;
// #[cfg(feature = "hidapi")]
// pub mod sensor;
pub mod surface;
// pub mod timer;
// pub mod touch;
// pub mod url;
// pub mod version;
pub mod video;

// modules
// #[cfg(feature = "gfx")]
// pub mod gfx;
// #[cfg(feature = "image")]
// pub mod image;
// #[cfg(feature = "mixer")]
// pub mod mixer;
// #[cfg(feature = "ttf")]
// pub mod ttf;

mod common;
// // Export return types and such from the common module.
pub use crate::common::IntegerOrSdlError;
