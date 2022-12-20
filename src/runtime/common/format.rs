use std::{ffi::c_char, mem};

use crate::{ffi, TRUE};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Format {
    #[default]
    Unknown = ffi::RpsFormat_RPS_FORMAT_UNKNOWN as _,
    R32G32B32A32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_TYPELESS as _,
    R32G32B32A32Float = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_FLOAT as _,
    R32G32B32A32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_UINT as _,
    R32G32B32A32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32B32A32_SINT as _,
    R32G32B32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32B32_TYPELESS as _,
    R32G32B32Float = ffi::RpsFormat_RPS_FORMAT_R32G32B32_FLOAT as _,
    R32G32B32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32B32_UINT as _,
    R32G32B32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32B32_SINT as _,
    R16G16B16A16Typeless = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_TYPELESS as _,
    R16G16B16A16Float = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_FLOAT as _,
    R16G16B16A16Unorm = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UNORM as _,
    R16G16B16A16Uint = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_UINT as _,
    R16G16B16A16Snorm = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SNORM as _,
    R16G16B16A16Sint = ffi::RpsFormat_RPS_FORMAT_R16G16B16A16_SINT as _,
    R32G32Typeless = ffi::RpsFormat_RPS_FORMAT_R32G32_TYPELESS as _,
    R32G32Float = ffi::RpsFormat_RPS_FORMAT_R32G32_FLOAT as _,
    R32G32Uint = ffi::RpsFormat_RPS_FORMAT_R32G32_UINT as _,
    R32G32Sint = ffi::RpsFormat_RPS_FORMAT_R32G32_SINT as _,
    R32G8X24Typeless = ffi::RpsFormat_RPS_FORMAT_R32G8X24_TYPELESS as _,
    D32FloatS8X24Uint = ffi::RpsFormat_RPS_FORMAT_D32_FLOAT_S8X24_UINT as _,
    R32FloatX8X24Typeless = ffi::RpsFormat_RPS_FORMAT_R32_FLOAT_X8X24_TYPELESS as _,
    X32TypelessG8X24Uint = ffi::RpsFormat_RPS_FORMAT_X32_TYPELESS_G8X24_UINT as _,
    R10G10B10A2Typeless = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_TYPELESS as _,
    R10G10B10A2Unorm = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UNORM as _,
    R10G10B10A2Uint = ffi::RpsFormat_RPS_FORMAT_R10G10B10A2_UINT as _,
    R11G11B10Float = ffi::RpsFormat_RPS_FORMAT_R11G11B10_FLOAT as _,
    R8G8B8A8Typeless = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_TYPELESS as _,
    R8G8B8A8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM as _,
    R8G8B8A8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UNORM_SRGB as _,
    R8G8B8A8Uint = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_UINT as _,
    R8G8B8A8Snorm = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SNORM as _,
    R8G8B8A8Sint = ffi::RpsFormat_RPS_FORMAT_R8G8B8A8_SINT as _,
    R16G16Typeless = ffi::RpsFormat_RPS_FORMAT_R16G16_TYPELESS as _,
    R16G16Float = ffi::RpsFormat_RPS_FORMAT_R16G16_FLOAT as _,
    R16G16Unorm = ffi::RpsFormat_RPS_FORMAT_R16G16_UNORM as _,
    R16G16Uint = ffi::RpsFormat_RPS_FORMAT_R16G16_UINT as _,
    R16G16Snorm = ffi::RpsFormat_RPS_FORMAT_R16G16_SNORM as _,
    R16G16Sint = ffi::RpsFormat_RPS_FORMAT_R16G16_SINT as _,
    R32Typeless = ffi::RpsFormat_RPS_FORMAT_R32_TYPELESS as _,
    D32Float = ffi::RpsFormat_RPS_FORMAT_D32_FLOAT as _,
    R32Float = ffi::RpsFormat_RPS_FORMAT_R32_FLOAT as _,
    R32Uint = ffi::RpsFormat_RPS_FORMAT_R32_UINT as _,
    R32Sint = ffi::RpsFormat_RPS_FORMAT_R32_SINT as _,
    R24G8Typeless = ffi::RpsFormat_RPS_FORMAT_R24G8_TYPELESS as _,
    D24UnormS8Uint = ffi::RpsFormat_RPS_FORMAT_D24_UNORM_S8_UINT as _,
    R24UnormX8Typeless = ffi::RpsFormat_RPS_FORMAT_R24_UNORM_X8_TYPELESS as _,
    X24TypelessG8Uint = ffi::RpsFormat_RPS_FORMAT_X24_TYPELESS_G8_UINT as _,
    R8G8Typeless = ffi::RpsFormat_RPS_FORMAT_R8G8_TYPELESS as _,
    R8G8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8_UNORM as _,
    R8G8Uint = ffi::RpsFormat_RPS_FORMAT_R8G8_UINT as _,
    R8G8Snorm = ffi::RpsFormat_RPS_FORMAT_R8G8_SNORM as _,
    R8G8Sint = ffi::RpsFormat_RPS_FORMAT_R8G8_SINT as _,
    R16Typeless = ffi::RpsFormat_RPS_FORMAT_R16_TYPELESS as _,
    R16Float = ffi::RpsFormat_RPS_FORMAT_R16_FLOAT as _,
    D16Unorm = ffi::RpsFormat_RPS_FORMAT_D16_UNORM as _,
    R16Unorm = ffi::RpsFormat_RPS_FORMAT_R16_UNORM as _,
    R16Uint = ffi::RpsFormat_RPS_FORMAT_R16_UINT as _,
    R16Snorm = ffi::RpsFormat_RPS_FORMAT_R16_SNORM as _,
    R16Sint = ffi::RpsFormat_RPS_FORMAT_R16_SINT as _,
    R8Typeless = ffi::RpsFormat_RPS_FORMAT_R8_TYPELESS as _,
    R8Unorm = ffi::RpsFormat_RPS_FORMAT_R8_UNORM as _,
    R8Uint = ffi::RpsFormat_RPS_FORMAT_R8_UINT as _,
    R8Snorm = ffi::RpsFormat_RPS_FORMAT_R8_SNORM as _,
    R8Sint = ffi::RpsFormat_RPS_FORMAT_R8_SINT as _,
    A8Unorm = ffi::RpsFormat_RPS_FORMAT_A8_UNORM as _,
    R1Unorm = ffi::RpsFormat_RPS_FORMAT_R1_UNORM as _,
    R9G9B9E5Sharedexp = ffi::RpsFormat_RPS_FORMAT_R9G9B9E5_SHAREDEXP as _,
    R8G8B8G8Unorm = ffi::RpsFormat_RPS_FORMAT_R8G8_B8G8_UNORM as _,
    G8R8G8B8Unorm = ffi::RpsFormat_RPS_FORMAT_G8R8_G8B8_UNORM as _,
    Bc1Typeless = ffi::RpsFormat_RPS_FORMAT_BC1_TYPELESS as _,
    Bc1Unorm = ffi::RpsFormat_RPS_FORMAT_BC1_UNORM as _,
    Bc1UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC1_UNORM_SRGB as _,
    Bc2Typeless = ffi::RpsFormat_RPS_FORMAT_BC2_TYPELESS as _,
    Bc2Unorm = ffi::RpsFormat_RPS_FORMAT_BC2_UNORM as _,
    Bc2UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC2_UNORM_SRGB as _,
    Bc3Typeless = ffi::RpsFormat_RPS_FORMAT_BC3_TYPELESS as _,
    Bc3Unorm = ffi::RpsFormat_RPS_FORMAT_BC3_UNORM as _,
    Bc3UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC3_UNORM_SRGB as _,
    Bc4Typeless = ffi::RpsFormat_RPS_FORMAT_BC4_TYPELESS as _,
    Bc4Unorm = ffi::RpsFormat_RPS_FORMAT_BC4_UNORM as _,
    Bc4Snorm = ffi::RpsFormat_RPS_FORMAT_BC4_SNORM as _,
    Bc5Typeless = ffi::RpsFormat_RPS_FORMAT_BC5_TYPELESS as _,
    Bc5Unorm = ffi::RpsFormat_RPS_FORMAT_BC5_UNORM as _,
    Bc5Snorm = ffi::RpsFormat_RPS_FORMAT_BC5_SNORM as _,
    B5G6R5Unorm = ffi::RpsFormat_RPS_FORMAT_B5G6R5_UNORM as _,
    B5G5R5A1Unorm = ffi::RpsFormat_RPS_FORMAT_B5G5R5A1_UNORM as _,
    B8G8R8A8Unorm = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM as _,
    B8G8R8X8Unorm = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM as _,
    R10G10B10XrBiasA2Unorm = ffi::RpsFormat_RPS_FORMAT_R10G10B10_XR_BIAS_A2_UNORM as _,
    B8G8R8A8Typeless = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_TYPELESS as _,
    B8G8R8A8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_B8G8R8A8_UNORM_SRGB as _,
    B8G8R8X8Typeless = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_TYPELESS as _,
    B8G8R8X8UnormSrgb = ffi::RpsFormat_RPS_FORMAT_B8G8R8X8_UNORM_SRGB as _,
    Bc6HTypeless = ffi::RpsFormat_RPS_FORMAT_BC6H_TYPELESS as _,
    Bc6HUf16 = ffi::RpsFormat_RPS_FORMAT_BC6H_UF16 as _,
    Bc6HSf16 = ffi::RpsFormat_RPS_FORMAT_BC6H_SF16 as _,
    Bc7Typeless = ffi::RpsFormat_RPS_FORMAT_BC7_TYPELESS as _,
    Bc7Unorm = ffi::RpsFormat_RPS_FORMAT_BC7_UNORM as _,
    Bc7UnormSrgb = ffi::RpsFormat_RPS_FORMAT_BC7_UNORM_SRGB as _,
    Ayuv = ffi::RpsFormat_RPS_FORMAT_AYUV as _,
    Y410 = ffi::RpsFormat_RPS_FORMAT_Y410 as _,
    Y416 = ffi::RpsFormat_RPS_FORMAT_Y416 as _,
    Nv12 = ffi::RpsFormat_RPS_FORMAT_NV12 as _,
    P010 = ffi::RpsFormat_RPS_FORMAT_P010 as _,
    P016 = ffi::RpsFormat_RPS_FORMAT_P016 as _,
    _420Opaque = ffi::RpsFormat_RPS_FORMAT_420_OPAQUE as _,
    Yuy2 = ffi::RpsFormat_RPS_FORMAT_YUY2 as _,
    Y210 = ffi::RpsFormat_RPS_FORMAT_Y210 as _,
    Y216 = ffi::RpsFormat_RPS_FORMAT_Y216 as _,
    Nv11 = ffi::RpsFormat_RPS_FORMAT_NV11 as _,
    Ai44 = ffi::RpsFormat_RPS_FORMAT_AI44 as _,
    Ia44 = ffi::RpsFormat_RPS_FORMAT_IA44 as _,
    P8 = ffi::RpsFormat_RPS_FORMAT_P8 as _,
    A8P8 = ffi::RpsFormat_RPS_FORMAT_A8P8 as _,
    B4G4R4A4Unorm = ffi::RpsFormat_RPS_FORMAT_B4G4R4A4_UNORM as _,
    Count = ffi::RpsFormat_RPS_FORMAT_COUNT as _
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
