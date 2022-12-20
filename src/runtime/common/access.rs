use bitflags::bitflags;

use crate::{ffi, utils::assert_size_and_align, Format, ResourceId, SubresourceRange};
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlags : u32 {
        const UNKNOWN = ffi::RpsAccessFlagBits_RPS_ACCESS_UNKNOWN as _;
        const INDIRECT_ARGS = ffi::RpsAccessFlagBits_RPS_ACCESS_INDIRECT_ARGS_BIT as _;
        const INDEX_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_INDEX_BUFFER_BIT as _;
        const VERTEX_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_VERTEX_BUFFER_BIT as _;
        const CONSTANT_BUFFER = ffi::RpsAccessFlagBits_RPS_ACCESS_CONSTANT_BUFFER_BIT as _;
        const SHADER_RESOURCE = ffi::RpsAccessFlagBits_RPS_ACCESS_SHADER_RESOURCE_BIT as _;
        const UNORDERED_ACCESS = ffi::RpsAccessFlagBits_RPS_ACCESS_UNORDERED_ACCESS_BIT as _;
        const SHADING_RATE = ffi::RpsAccessFlagBits_RPS_ACCESS_SHADING_RATE_BIT as _;

        const RENDER_TARGET = ffi::RpsAccessFlagBits_RPS_ACCESS_RENDER_TARGET_BIT as _;
        const DEPTH_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_READ_BIT as _;
        const DEPTH_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_WRITE_BIT as _;
        const STENCIL_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL_READ_BIT as _;
        const STENCIL_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL_WRITE_BIT as _;
        const STREAM_OUT = ffi::RpsAccessFlagBits_RPS_ACCESS_STREAM_OUT_BIT as _;
        const COPY_SRC = ffi::RpsAccessFlagBits_RPS_ACCESS_COPY_SRC_BIT as _;
        const COPY_DEST = ffi::RpsAccessFlagBits_RPS_ACCESS_COPY_DEST_BIT as _;
        const RESOLVE_SRC = ffi::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_SRC_BIT as _;
        const RESOLVE_DEST = ffi::RpsAccessFlagBits_RPS_ACCESS_RESOLVE_DEST_BIT as _;
        const RAYTRACING_AS_BUILD = ffi::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_BUILD_BIT as _;

        const RAYTRACING_AS_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_RAYTRACING_AS_READ_BIT as _;
        const PRESENT = ffi::RpsAccessFlagBits_RPS_ACCESS_PRESENT_BIT as _;
        const CPU_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_CPU_READ_BIT as _;
        const CPU_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_CPU_WRITE_BIT as _;

        const RENDER_PASS = ffi::RpsAccessFlagBits_RPS_ACCESS_RENDER_PASS as _;
        const BEFORE = ffi::RpsAccessFlagBits_RPS_ACCESS_BEFORE_BIT as _;
        const AFTER = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH as _;
        const CLEAR = ffi::RpsAccessFlagBits_RPS_ACCESS_CLEAR_BIT as _;
        const DISCARD_OLD_DATA = ffi::RpsAccessFlagBits_RPS_ACCESS_DISCARD_OLD_DATA_BIT as _;
        const RELAXED_ORDER = ffi::RpsAccessFlagBits_RPS_ACCESS_RELAXED_ORDER_BIT as _;
        const NO_VIEW = ffi::RpsAccessFlagBits_RPS_ACCESS_NO_VIEW_BIT as _;

        const PREDICATION = ffi::RpsAccessFlagBits_RPS_ACCESS_PREDICATION_BIT as _;
        const DEPTH = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH as _;
        const STENCIL = ffi::RpsAccessFlagBits_RPS_ACCESS_STENCIL as _;
        const DEPTH_STENCIL_READ = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_READ as _;
        const DEPTH_STENCIL_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL_WRITE as _;
        const DEPTH_STENCIL = ffi::RpsAccessFlagBits_RPS_ACCESS_DEPTH_STENCIL as _;
        const ALL_GPU_WRITE = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_WRITE as _;
        const ALL_GPU_READONLY = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU_READONLY as _;
        const ALL_GPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_GPU as _;
        const ALL_CPU = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_CPU as _;
        const ALL_ACCESS_MASK = ffi::RpsAccessFlagBits_RPS_ACCESS_ALL_ACCESS_MASK as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ShaderStage : u32 {
        const NONE = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_NONE as _;
        const VERTEX = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_VS as _;
        const PIXEL = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_PS as _;
        const GEOMETRY = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_GS as _;
        const COMPUTE = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_CS as _;
        const HULL = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_HS as _;
        const DOMAIN = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_DS as _;
        const RAYTRACING = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_RAYTRACING as _;
        const AMPLIFICATION = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_AS as _;
        const MESH = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_MS as _;
        const ALL = ffi::RpsShaderStageBits_RPS_SHADER_STAGE_ALL as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AccessAttr {
    pub access_flags: AccessFlags,
    pub access_stages: ShaderStage
}

assert_size_and_align!(AccessAttr, ffi::RpsAccessAttr);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Semantic {
    #[default]
    Unspecified = ffi::RpsSemantic_RPS_SEMANTIC_UNSPECIFIED as _,
    VertexShader = ffi::RpsSemantic_RPS_SEMANTIC_VERTEX_SHADER as _,
    PixelShader = ffi::RpsSemantic_RPS_SEMANTIC_PIXEL_SHADER as _,
    GeometryShader = ffi::RpsSemantic_RPS_SEMANTIC_GEOMETRY_SHADER as _,
    ComputeShader = ffi::RpsSemantic_RPS_SEMANTIC_COMPUTE_SHADER as _,
    HullShader = ffi::RpsSemantic_RPS_SEMANTIC_HULL_SHADER as _,
    DomainShader = ffi::RpsSemantic_RPS_SEMANTIC_DOMAIN_SHADER as _,
    RaytracingPipeline = ffi::RpsSemantic_RPS_SEMANTIC_RAYTRACING_PIPELINE as _,
    AmplificationShader = ffi::RpsSemantic_RPS_SEMANTIC_AMPLIFICATION_SHADER as _,
    MeshShader = ffi::RpsSemantic_RPS_SEMANTIC_MESH_SHADER as _,
    VertexLayout = ffi::RpsSemantic_RPS_SEMANTIC_VERTEX_LAYOUT as _,
    StreamOutLayout = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_LAYOUT as _,
    StreamOutDesc = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_DESC as _,
    BlendState = ffi::RpsSemantic_RPS_SEMANTIC_BLEND_STATE as _,
    RenderTargetBlend = ffi::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET_BLEND as _,
    DepthStencilState = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_STATE as _,
    RasterizerState = ffi::RpsSemantic_RPS_SEMANTIC_RASTERIZER_STATE as _,
    DynamicStateBeginOrViewport = ffi::RpsSemantic_RPS_SEMANTIC_DYNAMIC_STATE_BEGIN as _,
    Scissor = ffi::RpsSemantic_RPS_SEMANTIC_SCISSOR as _,
    PrimitiveTopology = ffi::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_TOPOLOGY as _,
    PatchControlPoints = ffi::RpsSemantic_RPS_SEMANTIC_PATCH_CONTROL_POINTS as _,
    PrimitiveStripCutIndex = ffi::RpsSemantic_RPS_SEMANTIC_PRIMITIVE_STRIP_CUT_INDEX as _,
    BlendFactor = ffi::RpsSemantic_RPS_SEMANTIC_BLEND_FACTOR as _,
    StencilRef = ffi::RpsSemantic_RPS_SEMANTIC_STENCIL_REF as _,
    DepthBounds = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_BOUNDS as _,
    SampleLocation = ffi::RpsSemantic_RPS_SEMANTIC_SAMPLE_LOCATION as _,
    ShadingRate = ffi::RpsSemantic_RPS_SEMANTIC_SHADING_RATE as _,
    ColorClearValue = ffi::RpsSemantic_RPS_SEMANTIC_COLOR_CLEAR_VALUE as _,
    DepthClearValue = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_CLEAR_VALUE as _,
    StencilClearValue = ffi::RpsSemantic_RPS_SEMANTIC_STENCIL_CLEAR_VALUE as _,
    ResourceBindingBeginOrVertexBuffer = ffi::RpsSemantic_RPS_SEMANTIC_RESOURCE_BINDING_BEGIN as _,
    IndexBuffer = ffi::RpsSemantic_RPS_SEMANTIC_INDEX_BUFFER as _,
    IndirectArgs = ffi::RpsSemantic_RPS_SEMANTIC_INDIRECT_ARGS as _,
    StreamOutBuffer = ffi::RpsSemantic_RPS_SEMANTIC_STREAM_OUT_BUFFER as _,
    IndirectCount = ffi::RpsSemantic_RPS_SEMANTIC_INDIRECT_COUNT as _,
    RenderTarget = ffi::RpsSemantic_RPS_SEMANTIC_RENDER_TARGET as _,
    DepthStencilTarget = ffi::RpsSemantic_RPS_SEMANTIC_DEPTH_STENCIL_TARGET as _,
    ShadingRateImage = ffi::RpsSemantic_RPS_SEMANTIC_SHADING_RATE_IMAGE as _,
    ResolveTarget = ffi::RpsSemantic_RPS_SEMANTIC_RESOLVE_TARGET as _,
    UserResourceBinding = ffi::RpsSemantic_RPS_SEMANTIC_USER_RESOURCE_BINDING as _
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SemanticAttr {
    pub semantic: Semantic,
    pub semantic_index: u32
}

assert_size_and_align!(SemanticAttr, ffi::RpsSemanticAttr);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ResourceViewFlags : u32 {
        const NONE = ffi::RpsResourceViewFlagBits_RPS_RESOURCE_VIEW_FLAG_NONE;
        const CUBEMAP = ffi::RpsResourceViewFlagBits_RPS_RESOURCE_VIEW_FLAG_CUBEMAP_BIT;
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

assert_size_and_align!(ResourceView, ffi::RpsResourceView);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImageView {
    pub base: ResourceView,
    pub subresource_range: SubresourceRange,
    pub min_lod_clamp: f32,
    pub component_mapping: u32
}

assert_size_and_align!(ImageView, ffi::RpsImageView);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ResourceViewComponentMapping {
    #[default]
    R = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_R as _,
    G = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_G as _,
    B = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_B as _,
    A = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_A as _,
    Zero = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_ZERO as _,
    One = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_ONE as _,
    Default = ffi::RpsResourceViewComponentMapping_RPS_RESOURCE_VIEW_COMPONENT_MAPPING_DEFAULT as _
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

assert_size_and_align!(BufferView, ffi::RpsBufferView);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RuntimeBuiltInTypeIds {
    #[default]
    ImageView = ffi::RpsRuntimeBuiltInTypeIds_RPS_TYPE_IMAGE_VIEW as _,
    BufferView = ffi::RpsRuntimeBuiltInTypeIds_RPS_TYPE_BUFFER_VIEW as _
}
