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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PrimitiveTopology {
    #[default]
    Undefined = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_UNDEFINED as _,
    PointList = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_POINTLIST as _,
    LineList = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST as _,
    LineStrip = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP as _,
    TriangleList = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST as _,
    TriangleStrip = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP as _,
    LineListAdj = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINELIST_ADJ as _,
    LineStripAdj = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ as _,
    TriangleListAdj = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ as _,
    TriangleStripAdj = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ as _,
    PatchList = ffi::RpsPrimitiveTopology_RPS_PRIMITIVE_TOPOLOGY_PATCHLIST as _
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ResolveMode {
    #[default]
    Average = ffi::RpsResolveMode_RPS_RESOLVE_MODE_AVERAGE as _,
    Min = ffi::RpsResolveMode_RPS_RESOLVE_MODE_MIN as _,
    Max = ffi::RpsResolveMode_RPS_RESOLVE_MODE_MAX as _,
    EncodeSamplerFeedback = ffi::RpsResolveMode_RPS_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK as _,
    DecodeSamplerFeedback = ffi::RpsResolveMode_RPS_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK as _
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
