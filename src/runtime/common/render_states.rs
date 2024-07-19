use std::mem;

use crate::{sys, utils::assert_size_and_align};

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

assert_size_and_align!(Viewport, sys::RpsViewport);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

assert_size_and_align!(Rect, sys::RpsRect);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrimitiveTopology(u32);

impl PrimitiveTopology {
    pub const UNDEFINED: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_UNDEFINED as _);
    pub const POINTLIST: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_POINTLIST as _);
    pub const LINELIST: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST as _);
    pub const LINESTRIP: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP as _);
    pub const TRIANGLELIST: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST as _);
    pub const TRIANGLESTRIP: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP as _);
    pub const LINELIST_ADJ: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST_ADJ as _);
    pub const LINESTRIP_ADJ: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ as _);
    pub const TRIANGLELIST_ADJ: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ as _);
    pub const TRIANGLESTRIP_ADJ: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ as _);
    pub const PATCHLIST: Self = Self(sys::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_PATCHLIST as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResolveMode(u32);

impl ResolveMode {
    pub const AVERAGE: Self = Self(sys::RpsResolveMode_RPS_RESOLVE_MODE_AVERAGE as _);
    pub const MIN: Self = Self(sys::RpsResolveMode_RPS_RESOLVE_MODE_MIN as _);
    pub const MAX: Self = Self(sys::RpsResolveMode_RPS_RESOLVE_MODE_MAX as _);
    pub const ENCODE_SAMPLER_FEEDBACK: Self = Self(sys::RpsResolveMode_RPS_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK as _);
    pub const DECODE_SAMPLER_FEEDBACK: Self = Self(sys::RpsResolveMode_RPS_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK as _);
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

assert_size_and_align!(CmdViewportInfo, sys::RpsCmdViewportInfo);
