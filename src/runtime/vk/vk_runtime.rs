use std::{mem, mem::MaybeUninit};

use ash::vk;
use bitflags::bitflags;

use crate::{
    ffi, result_from_ffi, utils::assert_size_and_align, CmdCallbackContext, Device, DeviceCreateInfo, Format, RpsResult, RuntimeCommandBuffer, RuntimeDeviceCreateInfo, RuntimeHeap,
    RuntimeResource
};
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct VKRuntimeFlags: u32 {
        const NONE = ffi::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_NONE as _;
        const PREFER_RENDER_PASS = ffi::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_PREFER_RENDER_PASS as _;
        const DONT_FLIP_VIEWPORT = ffi::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_DONT_FLIP_VIEWPORT as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VKRuntimeDeviceCreateInfo {
    pub device_create_info: *const DeviceCreateInfo,
    pub runtime_create_info: *const RuntimeDeviceCreateInfo,
    pub vk_device: vk::Device,
    pub vk_physical_device: vk::PhysicalDevice,
    pub flags: VKRuntimeFlags
}

impl Default for VKRuntimeDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(VKRuntimeDeviceCreateInfo, ffi::RpsVKRuntimeDeviceCreateInfo);

#[inline]
pub unsafe fn vk_runtime_device_create(create_info: *const VKRuntimeDeviceCreateInfo) -> RpsResult<Device> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKRuntimeDeviceCreate(create_info.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_command_buffer_from_handle(command_buffer: RuntimeCommandBuffer) -> vk::CommandBuffer {
    mem::transmute(command_buffer)
}

#[inline]
pub unsafe fn vk_command_buffer_to_handle(command_buffer: vk::CommandBuffer) -> RuntimeCommandBuffer {
    mem::transmute(command_buffer)
}

#[inline]
pub unsafe fn vk_image_from_handle(resource: RuntimeResource) -> vk::Image {
    mem::transmute(resource)
}

#[inline]
pub unsafe fn vk_image_to_handle(image: vk::Image) -> RuntimeResource {
    mem::transmute(image)
}

#[inline]
pub unsafe fn vk_buffer_from_handle(resource: RuntimeResource) -> vk::Buffer {
    mem::transmute(resource)
}

#[inline]
pub unsafe fn vk_buffer_to_handle(buffer: vk::Buffer) -> RuntimeResource {
    mem::transmute(buffer)
}

#[inline]
pub unsafe fn vk_memory_from_memory(heap: RuntimeHeap) -> vk::DeviceMemory {
    mem::transmute(heap)
}

#[inline]
pub unsafe fn vk_memory_to_handle(device_memory: vk::DeviceMemory) -> RuntimeHeap {
    mem::transmute(device_memory)
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image_view_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    image_views: *mut vk::ImageView,
    num_image_views: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgImageViewArray(context.cast(), arg_index, src_array_offset, image_views, num_image_views))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image_view(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::ImageView> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdArgImageView(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkImageViewInfo {
    pub image_view: vk::ImageView,
    pub layout: vk::ImageLayout
}

assert_size_and_align!(VkImageViewInfo, ffi::RpsVkImageViewInfo);

#[inline]
pub unsafe fn vk_get_cmd_arg_image_view_info_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    image_view_infos: *mut VkImageViewInfo,
    num_image_view_infos: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgImageViewInfoArray(
        context.cast(),
        arg_index,
        src_array_offset,
        image_view_infos.cast(),
        num_image_view_infos
    ))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image_array(context: *const CmdCallbackContext, arg_index: u32, src_array_offset: u32, images: *mut vk::Image, num_images: u32) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgImageArray(context.cast(), arg_index, src_array_offset, images, num_images))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::Image> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdArgImage(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer_view_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    buffer_views: *mut vk::BufferView,
    num_buffer_views: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgBufferViewArray(context.cast(), arg_index, src_array_offset, buffer_views, num_buffer_views))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer_view(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::BufferView> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdArgBufferView(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer_array(context: *const CmdCallbackContext, arg_index: u32, src_array_offset: u32, buffers: *mut vk::Buffer, num_buffers: u32) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgBufferArray(context.cast(), arg_index, src_array_offset, buffers, num_buffers))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::Buffer> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdArgBuffer(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkDeviceMemoryRange {
    pub memory: vk::DeviceMemory,
    pub offset: usize,
    pub size: usize
}

assert_size_and_align!(VkDeviceMemoryRange, ffi::RpsVkDeviceMemoryRange);

#[inline]
pub unsafe fn vk_get_cmd_arg_gpu_memory_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    memory_ranges: *mut VkDeviceMemoryRange,
    num_ranges: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsVKGetCmdArgGpuMemoryArray(context.cast(), arg_index, src_array_offset, memory_ranges.cast(), num_ranges))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_gpu_memory(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<VkDeviceMemoryRange> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdArgGpuMemory(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_get_cmd_render_pass(context: *const CmdCallbackContext) -> RpsResult<vk::RenderPass> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsVKGetCmdRenderPass(context.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn format_to_vk(format: Format) -> vk::Format {
    ffi::rpsFormatToVK(mem::transmute(format))
}

#[inline]
pub unsafe fn format_from_vk(format: vk::Format) -> Format {
    mem::transmute(ffi::rpsFormatFromVK(format))
}
