use std::{mem, mem::MaybeUninit};

use ash::{khr::dynamic_rendering, vk};
use bitflags::bitflags;

use crate::{
    result_from_ffi, sys, utils::assert_size_and_align, CmdCallbackContext, Device, DeviceCreateInfo, Format, RpsResult, RuntimeCommandBuffer, RuntimeDeviceCreateInfo, RuntimeHeap,
    RuntimeResource
};
bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct VKRuntimeFlags: u32 {
        const NONE = sys::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_NONE as _;
        const PREFER_RENDER_PASS = sys::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_PREFER_RENDER_PASS as _;
        const DONT_FLIP_VIEWPORT = sys::RpsVKRuntimeFlagBits_RPS_VK_RUNTIME_FLAG_DONT_FLIP_VIEWPORT as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VKFunctions {
    pub vk_get_physical_device_properties: vk::PFN_vkGetPhysicalDeviceProperties,
    pub vk_get_physical_device_memory_properties: vk::PFN_vkGetPhysicalDeviceMemoryProperties,
    pub vk_create_image: vk::PFN_vkCreateImage,
    pub vk_destroy_image: vk::PFN_vkDestroyImage,
    pub vk_bind_image_memory: vk::PFN_vkBindImageMemory,
    pub vk_get_image_memory_requirements: vk::PFN_vkGetImageMemoryRequirements,
    pub vk_create_buffer: vk::PFN_vkCreateBuffer,
    pub vk_destroy_buffer: vk::PFN_vkDestroyBuffer,
    pub vk_bind_buffer_memory: vk::PFN_vkBindBufferMemory,
    pub vk_get_buffer_memory_requirements: vk::PFN_vkGetBufferMemoryRequirements,
    pub vk_create_framebuffer: vk::PFN_vkCreateFramebuffer,
    pub vk_destroy_framebuffer: vk::PFN_vkDestroyFramebuffer,
    pub vk_create_render_pass: vk::PFN_vkCreateRenderPass,
    pub vk_destroy_render_pass: vk::PFN_vkDestroyRenderPass,
    pub vk_create_buffer_view: vk::PFN_vkCreateBufferView,
    pub vk_destroy_buffer_view: vk::PFN_vkDestroyBufferView,
    pub vk_create_image_view: vk::PFN_vkCreateImageView,
    pub vk_destroy_image_view: vk::PFN_vkDestroyImageView,
    pub vk_allocate_memory: vk::PFN_vkAllocateMemory,
    pub vk_free_memory: vk::PFN_vkFreeMemory,
    pub vk_cmd_begin_render_pass: vk::PFN_vkCmdBeginRenderPass,
    pub vk_cmd_end_render_pass: vk::PFN_vkCmdEndRenderPass,
    pub vk_cmd_set_viewport: vk::PFN_vkCmdSetViewport,
    pub vk_cmd_set_scissor: vk::PFN_vkCmdSetScissor,
    pub vk_cmd_pipeline_barrier: vk::PFN_vkCmdPipelineBarrier,
    pub vk_cmd_clear_color_image: vk::PFN_vkCmdClearColorImage,
    pub vk_cmd_clear_depth_stencil_image: vk::PFN_vkCmdClearDepthStencilImage,
    pub vk_cmd_copy_image: vk::PFN_vkCmdCopyImage,
    pub vk_cmd_copy_buffer: vk::PFN_vkCmdCopyBuffer,
    pub vk_cmd_copy_image_to_buffer: vk::PFN_vkCmdCopyImageToBuffer,
    pub vk_cmd_copy_buffer_to_image: vk::PFN_vkCmdCopyBufferToImage,
    pub vk_cmd_resolve_image: vk::PFN_vkCmdResolveImage,
    pub vk_cmd_begin_rendering: Option<vk::PFN_vkCmdBeginRendering>,
    pub vk_cmd_end_rendering: Option<vk::PFN_vkCmdEndRendering>
}

assert_size_and_align!(VKFunctions, sys::RpsVKFunctions);

impl VKFunctions {
    pub unsafe fn new(instance: &ash::Instance, physical_device: vk::PhysicalDevice, device: &ash::Device) -> Self {
        let properties = instance.get_physical_device_properties(physical_device);

        let (cmd_begin_rendering, cmd_end_rendering) = if properties.api_version < vk::API_VERSION_1_3 {
            let properties = instance.enumerate_device_extension_properties(physical_device).unwrap_or_default();

            let dynamic_rendering_supported = properties
                .iter()
                .any(|e| libc::strcmp(e.extension_name.as_ptr(), b"VK_KHR_dynamic_rendering\0".as_ptr().cast()) == 0);

            if dynamic_rendering_supported {
                let dynamic_rendering = dynamic_rendering::Device::new(instance, device);

                (Some(dynamic_rendering.fp().cmd_begin_rendering_khr), Some(dynamic_rendering.fp().cmd_end_rendering_khr))
            } else {
                (None, None)
            }
        } else {
            (Some(device.fp_v1_3().cmd_begin_rendering), Some(device.fp_v1_3().cmd_end_rendering))
        };

        Self {
            vk_get_physical_device_properties: instance.fp_v1_0().get_physical_device_properties,
            vk_get_physical_device_memory_properties: instance.fp_v1_0().get_physical_device_memory_properties,
            vk_create_image: device.fp_v1_0().create_image,
            vk_destroy_image: device.fp_v1_0().destroy_image,
            vk_bind_image_memory: device.fp_v1_0().bind_image_memory,
            vk_get_image_memory_requirements: device.fp_v1_0().get_image_memory_requirements,
            vk_create_buffer: device.fp_v1_0().create_buffer,
            vk_destroy_buffer: device.fp_v1_0().destroy_buffer,
            vk_bind_buffer_memory: device.fp_v1_0().bind_buffer_memory,
            vk_get_buffer_memory_requirements: device.fp_v1_0().get_buffer_memory_requirements,
            vk_create_framebuffer: device.fp_v1_0().create_framebuffer,
            vk_destroy_framebuffer: device.fp_v1_0().destroy_framebuffer,
            vk_create_render_pass: device.fp_v1_0().create_render_pass,
            vk_destroy_render_pass: device.fp_v1_0().destroy_render_pass,
            vk_create_buffer_view: device.fp_v1_0().create_buffer_view,
            vk_destroy_buffer_view: device.fp_v1_0().destroy_buffer_view,
            vk_create_image_view: device.fp_v1_0().create_image_view,
            vk_destroy_image_view: device.fp_v1_0().destroy_image_view,
            vk_allocate_memory: device.fp_v1_0().allocate_memory,
            vk_free_memory: device.fp_v1_0().free_memory,
            vk_cmd_begin_render_pass: device.fp_v1_0().cmd_begin_render_pass,
            vk_cmd_end_render_pass: device.fp_v1_0().cmd_end_render_pass,
            vk_cmd_set_viewport: device.fp_v1_0().cmd_set_viewport,
            vk_cmd_set_scissor: device.fp_v1_0().cmd_set_scissor,
            vk_cmd_pipeline_barrier: device.fp_v1_0().cmd_pipeline_barrier,
            vk_cmd_clear_color_image: device.fp_v1_0().cmd_clear_color_image,
            vk_cmd_clear_depth_stencil_image: device.fp_v1_0().cmd_clear_depth_stencil_image,
            vk_cmd_copy_image: device.fp_v1_0().cmd_copy_image,
            vk_cmd_copy_buffer: device.fp_v1_0().cmd_copy_buffer,
            vk_cmd_copy_image_to_buffer: device.fp_v1_0().cmd_copy_image_to_buffer,
            vk_cmd_copy_buffer_to_image: device.fp_v1_0().cmd_copy_buffer_to_image,
            vk_cmd_resolve_image: device.fp_v1_0().cmd_resolve_image,
            vk_cmd_begin_rendering: cmd_begin_rendering,
            vk_cmd_end_rendering: cmd_end_rendering
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VKRuntimeDeviceCreateInfo {
    pub device_create_info: *const DeviceCreateInfo,
    pub runtime_create_info: *const RuntimeDeviceCreateInfo,
    pub vk_device: vk::Device,
    pub vk_physical_device: vk::PhysicalDevice,
    pub flags: VKRuntimeFlags,
    pub vk_functions: *const VKFunctions
}

impl Default for VKRuntimeDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(VKRuntimeDeviceCreateInfo, sys::RpsVKRuntimeDeviceCreateInfo);

#[inline]
pub unsafe fn vk_runtime_device_create(create_info: *const VKRuntimeDeviceCreateInfo) -> RpsResult<Device> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKRuntimeDeviceCreate(create_info.cast(), &mut result as *mut _ as *mut _))?;
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
    result_from_ffi(sys::rpsVKGetCmdArgImageViewArray(context.cast(), arg_index, src_array_offset, image_views, num_image_views))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image_view(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::ImageView> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdArgImageView(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkImageViewInfo {
    pub image_view: vk::ImageView,
    pub layout: vk::ImageLayout
}

assert_size_and_align!(VkImageViewInfo, sys::RpsVkImageViewInfo);

#[inline]
pub unsafe fn vk_get_cmd_arg_image_view_info_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    image_view_infos: *mut VkImageViewInfo,
    num_image_view_infos: u32
) -> RpsResult<()> {
    result_from_ffi(sys::rpsVKGetCmdArgImageViewInfoArray(
        context.cast(),
        arg_index,
        src_array_offset,
        image_view_infos.cast(),
        num_image_view_infos
    ))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image_array(context: *const CmdCallbackContext, arg_index: u32, src_array_offset: u32, images: *mut vk::Image, num_images: u32) -> RpsResult<()> {
    result_from_ffi(sys::rpsVKGetCmdArgImageArray(context.cast(), arg_index, src_array_offset, images, num_images))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_image(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::Image> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdArgImage(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
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
    result_from_ffi(sys::rpsVKGetCmdArgBufferViewArray(context.cast(), arg_index, src_array_offset, buffer_views, num_buffer_views))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer_view(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::BufferView> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdArgBufferView(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer_array(context: *const CmdCallbackContext, arg_index: u32, src_array_offset: u32, buffers: *mut vk::Buffer, num_buffers: u32) -> RpsResult<()> {
    result_from_ffi(sys::rpsVKGetCmdArgBufferArray(context.cast(), arg_index, src_array_offset, buffers, num_buffers))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_buffer(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<vk::Buffer> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdArgBuffer(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct VkDeviceMemoryRange {
    pub memory: vk::DeviceMemory,
    pub offset: usize,
    pub size: usize
}

assert_size_and_align!(VkDeviceMemoryRange, sys::RpsVkDeviceMemoryRange);

#[inline]
pub unsafe fn vk_get_cmd_arg_gpu_memory_array(
    context: *const CmdCallbackContext,
    arg_index: u32,
    src_array_offset: u32,
    memory_ranges: *mut VkDeviceMemoryRange,
    num_ranges: u32
) -> RpsResult<()> {
    result_from_ffi(sys::rpsVKGetCmdArgGpuMemoryArray(context.cast(), arg_index, src_array_offset, memory_ranges.cast(), num_ranges))
}

#[inline]
pub unsafe fn vk_get_cmd_arg_gpu_memory(context: *const CmdCallbackContext, arg_index: u32) -> RpsResult<VkDeviceMemoryRange> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdArgGpuMemory(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn vk_get_cmd_render_pass(context: *const CmdCallbackContext) -> RpsResult<vk::RenderPass> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(sys::rpsVKGetCmdRenderPass(context.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn format_to_vk(format: Format) -> vk::Format {
    sys::rpsFormatToVK(mem::transmute(format))
}

#[inline]
pub unsafe fn format_from_vk(format: vk::Format) -> Format {
    mem::transmute(sys::rpsFormatFromVK(format))
}
