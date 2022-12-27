#![feature(concat_idents)]
#![allow(clippy::missing_safety_doc)]

mod core;

mod ffi {
    pub use render_pipeline_shaders_sys::*;
}

mod runtime;
mod utils;

pub use runtime::*;

pub use crate::core::*;
