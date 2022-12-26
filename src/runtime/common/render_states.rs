use std::mem;

use crate::{ffi, utils::assert_size_and_align};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_z: f32,
    pub max_z: f32
}

assert_size_and_align!(Viewport, ffi::RpsViewport);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

assert_size_and_align!(Rect, ffi::RpsRect);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrimitiveTopology(u32);

impl PrimitiveTopology {
    pub const UNDEFINED: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_UNDEFINED);
    pub const POINTLIST: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_POINTLIST);
    pub const LINELIST: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST);
    pub const LINESTRIP: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP);
    pub const TRIANGLELIST: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    pub const TRIANGLESTRIP: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
    pub const LINELIST_ADJ: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST_ADJ);
    pub const LINESTRIP_ADJ: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ);
    pub const TRIANGLELIST_ADJ: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ);
    pub const TRIANGLESTRIP_ADJ: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ);
    pub const PATCHLIST: Self = Self(ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_PATCHLIST);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResolveMode(u32);

impl ResolveMode {
    pub const AVERAGE: Self = Self(ffi::RpsResolveMode_RPS_RESOLVE_MODE_AVERAGE);
    pub const MIN: Self = Self(ffi::RpsResolveMode_RPS_RESOLVE_MODE_MIN);
    pub const MAX: Self = Self(ffi::RpsResolveMode_RPS_RESOLVE_MODE_MAX);
    pub const ENCODE_SAMPLER_FEEDBACK: Self = Self(ffi::RpsResolveMode_RPS_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK);
    pub const DECODE_SAMPLER_FEEDBACK: Self = Self(ffi::RpsResolveMode_RPS_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CmdViewportInfo {
    pub default_render_area: Rect,
    pub num_viewports: u32,
    pub num_scissor_rects: u32,
    pub viewports: *const Viewport,
    pub scissor_rects: *const Rect
}

impl Default for CmdViewportInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(CmdViewportInfo, ffi::RpsCmdViewportInfo);
