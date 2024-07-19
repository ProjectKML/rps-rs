use bitflags::bitflags;

use crate::{sys, utils::assert_size_and_align, Format, ResourceId, SubresourceRange};
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlags : u32 {
        const UNKNOWN = sys::RpsAccessFlagBits_RPS_ACCESS_UNKNOWN as _;
        const INDIRECT_ARGS = sys::RpsAccessFlagBits_RPS_ACCESS_INDIRECT_ARGS_BIT as _;
        const INDEX_BUFFER = sys::RpsAccessFlagBits_RPS_ACCESS_INDEX_BUFFER_BIT as _;
        const VERTEX_BUFFER = sys::RpsAccessFlagBits_RPS_ACCESS_VERTEX_BUFFER_BIT as _;
        const CONSTANT_BUFFER = sys::RpsAccessFlagBits_RPS_ACCESS_CONSTANT_BUFFER_BIT as _;
        const SHADER_RESOURCE = sys::RpsAccessFlagBits_RPS_ACCESS_SHADER_RESOURCE_BIT as _;
        const UNORDERED_ACCESS = sys::RpsAccessFlagBits_RPS_ACCESS_UNORDERED_ACCESS_BIT as _;
        const SHADING_RATE = sys::RpsAccessFlagBits_RPS_ACCESS_SHADING_RATE_BIT as _;

        const RENDER_TARGET = sys::RpsAccessFlagBits_RPS_ACCESS_RENDER_TARGET_BIT as _;
        const DEPTH_READ = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH_READ_BIT as _;
        const DEPTH_WRITE = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH_WRITE_BIT as _;
        const STENCIL_READ = sys::RpsAccessFlagBits_RPS_ACCESS_STENCIL_READ_BIT as _;
        const STENCIL_WRITE = sys::RpsAccessFlagBits_RPS_ACCESS_STENCIL_WRITE_BIT as _;
        const STREAM_OUT = sys::RpsAccessFlagBits_RPS_ACCESS_STREAM_OUT_BIT as _;
        const COPY_SRC = sys::RpsAccessFlagBits_RPS_ACCESS_COPY_SRC_BIT as _;
        const COPY_DEST = sys::RpsAccessFlagBits_RPS_ACCESS_COPY_DEST_BIT as _;
        const RESOLVE_SRC = sys::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_SRC_BIT as _;
        const RESOLVE_DEST = sys::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_DEST_BIT as _;
        const RAYTRACING_AS_BUILD = sys::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_BUILD_BIT as _;

        const RAYTRACING_AS_READ = sys::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_READ_BIT as _;
        const PRESENT = sys::RpsAccessFlagBits_RPS_ACCESS_PRESENT_BIT as _;
        const CPU_READ = sys::RpsAccessFlagBits_RPS_ACCESS_CPU_READ_BIT as _;
        const CPU_WRITE = sys::RpsAccessFlagBits_RPS_ACCESS_CPU_WRITE_BIT as _;

        const DISCARD_DATA_BEFORE = sys::RpsAccessFlagBits_RPS_ACCESS_DISCARD_DATA_BEFORE_BIT as _;
        const DISCARD_DATA_AFTER = sys::RpsAccessFlagBits_RPS_ACCESS_DISCARD_DATA_AFTER_BIT as _;
        const STENCIL_DISCARD_DATA_BEFORE = sys::RpsAccessFlagBits_RPS_ACCESS_STENCIL_DISCARD_DATA_BEFORE_BIT as _;
        const STENCIL_DISCARD_DATA_AFTER = sys::RpsAccessFlagBits_RPS_ACCESS_STENCIL_DISCARD_DATA_AFTER_BIT as _;
        const BEFORE = sys::RpsAccessFlagBits_RPS_ACCESS_BEFORE_BIT as _;
        const AFTER = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH as _;
        const CLEAR = sys::RpsAccessFlagBits_RPS_ACCESS_CLEAR_BIT as _;
        const RENDER_PASS = sys::RpsAccessFlagBits_RPS_ACCESS_RENDER_PASS as _;
        const RELAXED_ORDER = sys::RpsAccessFlagBits_RPS_ACCESS_RELAXED_ORDER_BIT as _;
        const NO_VIEW = sys::RpsAccessFlagBits_RPS_ACCESS_NO_VIEW_BIT as _;

        const PREDICATION = sys::RpsAccessFlagBits_RPS_ACCESS_PREDICATION_BIT as _;
        const DEPTH = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH as _;
        const STENCIL = sys::RpsAccessFlagBits_RPS_ACCESS_STENCIL as _;
        const DEPTH_STENCIL_READ = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_READ as _;
        const DEPTH_STENCIL_WRITE = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_WRITE as _;
        const DEPTH_STENCIL = sys::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL as _;
        const ALL_GPU_WRITE = sys::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_WRITE as _;
        const ALL_GPU_READONLY = sys::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_READONLY as _;
        const ALL_GPU = sys::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU as _;
        const ALL_CPU = sys::RpsAccessFlagBits_RPS_ACCESS_ALL_CPU as _;
        const ALL_ACCESS_MASK = sys::RpsAccessFlagBits_RPS_ACCESS_ALL_ACCESS_MASK as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ShaderStage : u32 {
        const NONE = sys::RpsShaderStageBits_RPS_SHADER_STAGE_NONE as _;
        const VS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_VS as _;
        const PS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_PS as _;
        const GS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_GS as _;
        const CS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_CS as _;
        const HS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_HS as _;
        const DS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_DS as _;
        const RAYTRACING = sys::RpsShaderStageBits_RPS_SHADER_STAGE_RAYTRACING as _;
        const AS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_AS as _;
        const MS = sys::RpsShaderStageBits_RPS_SHADER_STAGE_MS as _;
        const ALL = sys::RpsShaderStageBits_RPS_SHADER_STAGE_ALL as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AccessAttr {
    pub access_flags: AccessFlags,
    pub access_stages: ShaderStage
}

assert_size_and_align!(AccessAttr, sys::RpsAccessAttr);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Semantic(u32);

impl Semantic {
    pub const UNSPECIFIED: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_UNSPECIFIED as _);
    pub const VERTEX_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_VERTEX_SHADER as _);
    pub const PIXEL_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_PIXEL_SHADER as _);
    pub const GEOMETRY_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_GEOMETRY_SHADER as _);
    pub const COMPUTE_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_COMPUTE_SHADER as _);
    pub const HULL_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_HULL_SHADER as _);
    pub const DOMAIN_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DOMAIN_SHADER as _);
    pub const RAYTRACING_PIPELINE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RAYTRACING_PIPELINE as _);
    pub const AMPLIFICATION_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_AMPLIFICATION_SHADER as _);
    pub const MESH_SHADER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_MESH_SHADER as _);
    pub const VERTEX_LAYOUT: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_VERTEX_LAYOUT as _);
    pub const STREAM_OUT_LAYOUT: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_LAYOUT as _);
    pub const STREAM_OUT_DESC: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_DESC as _);
    pub const BLEND_STATE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_BLEND_STATE as _);
    pub const RENDER_TARGET_BLEND: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET_BLEND as _);
    pub const DEPTH_STENCIL_STATE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_STATE as _);
    pub const RASTERIZER_STATE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RASTERIZER_STATE as _);
    pub const DYNAMIC_STATE_BEGIN: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DYNAMIC_STATE_BEGIN as _);
    pub const VIEWPORT: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_VIEWPORT as _);
    pub const SCISSOR: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_SCISSOR as _);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_TOPOLOGY as _);
    pub const PATCH_CONTROL_POINTS: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_PATCH_CONTROL_POINTS as _);
    pub const PRIMITIVE_STRIP_CUT_INDEX: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_STRIP_CUT_INDEX as _);
    pub const BLEND_FACTOR: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_BLEND_FACTOR as _);
    pub const STENCIL_REF: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_STENCIL_REF as _);
    pub const DEPTH_BOUNDS: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DEPTH_BOUNDS as _);
    pub const SAMPLE_LOCATION: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_SAMPLE_LOCATION as _);
    pub const SHADING_RATE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_SHADING_RATE as _);
    pub const COLOR_CLEAR_VALUE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_COLOR_CLEAR_VALUE as _);
    pub const DEPTH_CLEAR_VALUE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DEPTH_CLEAR_VALUE as _);
    pub const STENCIL_CLEAR_VALUE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_STENCIL_CLEAR_VALUE as _);
    pub const RESOURCE_BINDING_BEGIN: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RESOURCE_BINDING_BEGIN as _);
    pub const VERTEX_BUFFER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_VERTEX_BUFFER as _);
    pub const INDEX_BUFFER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_INDEX_BUFFER as _);
    pub const INDIRECT_ARGS: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_INDIRECT_ARGS as _);
    pub const STREAM_OUT_BUFFER: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_BUFFER as _);
    pub const INDIRECT_COUNT: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_INDIRECT_COUNT as _);
    pub const RENDER_TARGET: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET as _);
    pub const DEPTH_STENCIL_TARGET: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_TARGET as _);
    pub const SHADING_RATE_IMAGE: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_SHADING_RATE_IMAGE as _);
    pub const RESOLVE_TARGET: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_RESOLVE_TARGET as _);
    pub const USER_RESOURCE_BINDING: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_USER_RESOURCE_BINDING as _);
    pub const COUNT: Self = Self(sys::RpsSemantic_RPS_SEMANTIC_COUNT as _);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SemanticAttr {
    pub semantic: Semantic,
    pub semantic_index: u32
}

assert_size_and_align!(SemanticAttr, sys::RpsSemanticAttr);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ResourceViewFlags : u32 {
        const NONE = sys::RpsResourceViewFlagBits_RPS_RESOURCE_VIEW_FLAG_NONE as _;
        const CUBEMAP = sys::RpsResourceViewFlagBits_RPS_RESOURCE_VIEW_FLAG_CUBEMAP_BIT as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceView {
    pub resource_id: ResourceId,
    pub view_format: Format,
    pub temporal_layer: u32,
    pub flags: ResourceViewFlags
}

assert_size_and_align!(ResourceView, sys::RpsResourceView);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImageView {
    pub base: ResourceView,
    pub subresource_range: SubresourceRange,
    pub min_lod_clamp: f32,
    pub component_mapping: u32
}

assert_size_and_align!(ImageView, sys::RpsImageView);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceViewComponentMapping(u32);

impl ResourceViewComponentMapping {
    pub const R: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_R as _);
    pub const G: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_G as _);
    pub const B: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_B as _);
    pub const A: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_A as _);
    pub const ZERO: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_ZERO as _);
    pub const ONE: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_ONE as _);
    pub const DEFAULT: Self = Self(sys::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_DEFAULT as _);
}

#[inline]
pub const fn image_view_make_component_mapping(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32 & 0xFF) | ((g as u32 & 0xFF) << 8) | ((b as u32 & 0xFF) << 16) | ((a as u32 & 0xFF) << 24)
}

#[inline]
pub const fn image_view_get_component_mapping_channel_r(packed: u32) -> u8 {
    (packed & 0xff) as _
}

#[inline]
pub const fn image_view_get_component_mapping_channel_g(packed: u32) -> u8 {
    ((packed >> 8) & 0xff) as _
}

#[inline]
pub const fn image_view_get_component_mapping_channel_b(packed: u32) -> u8 {
    ((packed >> 16) & 0xff) as _
}

#[inline]
pub const fn image_view_get_component_mapping_channel_a(packed: u32) -> u8 {
    ((packed >> 24) & 0xff) as _
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BufferView {
    pub base: ResourceView,
    pub offset: u64,
    pub size_in_bytes: u64,
    pub stride: u32
}

assert_size_and_align!(BufferView, sys::RpsBufferView);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RuntimeBuiltInTypeIds(u32);

impl RuntimeBuiltInTypeIds {
    pub const IMAGE_VIEW: Self = Self(sys::RpsRuntimeBuiltInTypeIds_RPS_TYPE_IMAGE_VIEW as _);
    pub const BUFFER_VIEW: Self = Self(sys::RpsRuntimeBuiltInTypeIds_RPS_TYPE_BUFFER_VIEW as _);
}
