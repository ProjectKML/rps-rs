use std::{ffi::c_char, mem};

use crate::{ffi, TRUE};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Format(u32);

impl Format {
    pub const UNKNOWN: Self = Self(ffi::RpsFormat_RPS_FORMAT_UNKNOWN);
    pub const R32G32B32A32_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_TYPELESS);
    pub const R32G32B32A32_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_FLOAT);
    pub const R32G32B32A32_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_UINT);
    pub const R32G32B32A32_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_SINT);
    pub const R32G32B32_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32_TYPELESS);
    pub const R32G32B32_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32_FLOAT);
    pub const R32G32B32_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32_UINT);
    pub const R32G32B32_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32B32_SINT);
    pub const R16G16B16A16_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_TYPELESS);
    pub const R16G16B16A16_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_FLOAT);
    pub const R16G16B16A16_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UNORM);
    pub const R16G16B16A16_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UINT);
    pub const R16G16B16A16_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SNORM);
    pub const R16G16B16A16_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SINT);
    pub const R32G32_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32_TYPELESS);
    pub const R32G32_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32_FLOAT);
    pub const R32G32_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32_UINT);
    pub const R32G32_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G32_SINT);
    pub const R32G8X24_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32G8X24_TYPELESS);
    pub const D32_FLOAT_S8X24_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_D32_FLOAT_S8X24_UINT);
    pub const R32_FLOAT_X8X24_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32_FLOAT_X8X24_TYPELESS);
    pub const X32_TYPELESS_G8X24_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_X32_TYPELESS_G8X24_UINT);
    pub const R10G10B10A2_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_TYPELESS);
    pub const R10G10B10A2_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UNORM);
    pub const R10G10B10A2_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UINT);
    pub const R11G11B10_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R11G11B10_FLOAT);
    pub const R8G8B8A8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_TYPELESS);
    pub const R8G8B8A8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM);
    pub const R8G8B8A8_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM_SRGB);
    pub const R8G8B8A8_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UINT);
    pub const R8G8B8A8_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SNORM);
    pub const R8G8B8A8_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SINT);
    pub const R16G16_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_TYPELESS);
    pub const R16G16_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_FLOAT);
    pub const R16G16_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_UNORM);
    pub const R16G16_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_UINT);
    pub const R16G16_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_SNORM);
    pub const R16G16_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16G16_SINT);
    pub const R32_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32_TYPELESS);
    pub const D32_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_D32_FLOAT);
    pub const R32_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32_FLOAT);
    pub const R32_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32_UINT);
    pub const R32_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R32_SINT);
    pub const R24G8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R24G8_TYPELESS);
    pub const D24_UNORM_S8_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_D24_UNORM_S8_UINT);
    pub const R24_UNORM_X8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R24_UNORM_X8_TYPELESS);
    pub const X24_TYPELESS_G8_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_X24_TYPELESS_G8_UINT);
    pub const R8G8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_TYPELESS);
    pub const R8G8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_UNORM);
    pub const R8G8_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_UINT);
    pub const R8G8_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_SNORM);
    pub const R8G8_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_SINT);
    pub const R16_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_TYPELESS);
    pub const R16_FLOAT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_FLOAT);
    pub const D16_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_D16_UNORM);
    pub const R16_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_UNORM);
    pub const R16_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_UINT);
    pub const R16_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_SNORM);
    pub const R16_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R16_SINT);
    pub const R8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8_TYPELESS);
    pub const R8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8_UNORM);
    pub const R8_UINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8_UINT);
    pub const R8_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8_SNORM);
    pub const R8_SINT: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8_SINT);
    pub const A8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_A8_UNORM);
    pub const R9G9B9E5_SHAREDEXP: Self = Self(ffi::RpsFormat_RPS_FORMAT_R9G9B9E5_SHAREDEXP);
    pub const R8G8_B8G8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R8G8_B8G8_UNORM);
    pub const G8R8_G8B8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_G8R8_G8B8_UNORM);
    pub const BC1_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC1_TYPELESS);
    pub const BC1_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC1_UNORM);
    pub const BC1_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC1_UNORM_SRGB);
    pub const BC2_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC2_TYPELESS);
    pub const BC2_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC2_UNORM);
    pub const BC2_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC2_UNORM_SRGB);
    pub const BC3_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC3_TYPELESS);
    pub const BC3_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC3_UNORM);
    pub const BC3_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC3_UNORM_SRGB);
    pub const BC4_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC4_TYPELESS);
    pub const BC4_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC4_UNORM);
    pub const BC4_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC4_SNORM);
    pub const BC5_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC5_TYPELESS);
    pub const BC5_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC5_UNORM);
    pub const BC5_SNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC5_SNORM);
    pub const B5G6R5_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_B5G6R5_UNORM);
    pub const B5G5R5A1_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_B5G5R5A1_UNORM);
    pub const B8G8R8A8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM);
    pub const B8G8R8X8_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM);
    pub const R10G10B10_XR_BIAS_A2_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_R10G10B10_XR_BIAS_A2_UNORM);
    pub const B8G8R8A8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_TYPELESS);
    pub const B8G8R8A8_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM_SRGB);
    pub const B8G8R8X8_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_TYPELESS);
    pub const B8G8R8X8_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM_SRGB);
    pub const BC6H_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC6H_TYPELESS);
    pub const BC6H_UF16: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC6H_UF16);
    pub const BC6H_SF16: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC6H_SF16);
    pub const BC7_TYPELESS: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC7_TYPELESS);
    pub const BC7_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC7_UNORM);
    pub const BC7_UNORM_SRGB: Self = Self(ffi::RpsFormat_RPS_FORMAT_BC7_UNORM_SRGB);
    pub const AYUV: Self = Self(ffi::RpsFormat_RPS_FORMAT_AYUV);
    pub const Y410: Self = Self(ffi::RpsFormat_RPS_FORMAT_Y410);
    pub const Y416: Self = Self(ffi::RpsFormat_RPS_FORMAT_Y416);
    pub const NV12: Self = Self(ffi::RpsFormat_RPS_FORMAT_NV12);
    pub const P010: Self = Self(ffi::RpsFormat_RPS_FORMAT_P010);
    pub const P016: Self = Self(ffi::RpsFormat_RPS_FORMAT_P016);
    pub const _420_OPAQUE: Self = Self(ffi::RpsFormat_RPS_FORMAT_420_OPAQUE);
    pub const YUY2: Self = Self(ffi::RpsFormat_RPS_FORMAT_YUY2);
    pub const Y210: Self = Self(ffi::RpsFormat_RPS_FORMAT_Y210);
    pub const Y216: Self = Self(ffi::RpsFormat_RPS_FORMAT_Y216);
    pub const NV11: Self = Self(ffi::RpsFormat_RPS_FORMAT_NV11);
    pub const AI44: Self = Self(ffi::RpsFormat_RPS_FORMAT_AI44);
    pub const IA44: Self = Self(ffi::RpsFormat_RPS_FORMAT_IA44);
    pub const P8: Self = Self(ffi::RpsFormat_RPS_FORMAT_P8);
    pub const A8P8: Self = Self(ffi::RpsFormat_RPS_FORMAT_A8P8);
    pub const B4G4R4A4_UNORM: Self = Self(ffi::RpsFormat_RPS_FORMAT_B4G4R4A4_UNORM);
    pub const COUNT: Self = Self(ffi::RpsFormat_RPS_FORMAT_COUNT);
}

impl Format {
    #[inline]
    pub fn block_compressed(self) -> bool {
        unsafe { ffi::rpsFormatIsBlockCompressed(mem::transmute(self)) == TRUE }
    }

    #[inline]
    pub fn has_depth_stencil(self) -> bool {
        unsafe { ffi::rpsFormatHasDepthStencil(mem::transmute(self)) == TRUE }
    }

    #[inline]
    pub fn has_depth(self) -> bool {
        unsafe { ffi::rpsFormatHasDepth(mem::transmute(self)) == TRUE }
    }

    #[inline]
    pub fn has_stencil(self) -> bool {
        unsafe { ffi::rpsFormatHasStencil(mem::transmute(self)) == TRUE }
    }

    #[inline]
    pub fn depth_only(self) -> bool {
        unsafe { ffi::rpsFormatIsDepthOnly(mem::transmute(self)) == TRUE }
    }

    #[inline]
    pub fn element_bytes(self) -> u32 {
        unsafe { ffi::rpsGetFormatElementBytes(mem::transmute(self)) }
    }

    #[inline]
    pub fn name(self) -> *const c_char {
        unsafe { ffi::rpsFormatGetName(mem::transmute(self)) }
    }
}
