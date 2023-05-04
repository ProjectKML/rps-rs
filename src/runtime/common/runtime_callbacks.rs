use std::{
    ffi::{c_char, c_void},
    mem
};

use bitflags::bitflags;

use crate::{
    core::Result,
    ffi,
    utils::{assert_size_and_align, define_handle},
    AccessAttr, Bool, ClearValue, Format, GpuMemoryRequirement, HeapPlacement, RenderGraph, RenderGraphPhaseInfo, ResourceDesc, ResourceId, ResourceType, RuntimeCommandBuffer,
    RuntimeResource, Variable
};

define_handle!(RuntimeHeap);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpCreateHeapArgs {
    pub memory_type_index: u32,
    pub size: usize,
    pub alignment: usize,
    pub debug_name: *const c_char,
    pub runtime_heap: *mut RuntimeHeap
}

impl Default for RuntimeOpCreateHeapArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpCreateHeapArgs, ffi::RpsRuntimeOpCreateHeapArgs);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpDestroyHeapArgs {
    pub num_heaps: u32,
    pub heaps: *const RuntimeHeap
}

impl Default for RuntimeOpDestroyHeapArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpDestroyHeapArgs, ffi::RpsRuntimeOpDestroyHeapArgs);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RuntimeOpCreateResourceArgs {
    pub resource_id: ResourceId,
    pub desc: ResourceDesc,
    pub original_desc: Variable,
    pub clear_value: ClearValue,
    pub alloc_requirement: GpuMemoryRequirement,
    pub alloc_placement: HeapPlacement,
    pub all_accesses: AccessAttr,
    pub initial_access: AccessAttr,
    pub num_mutable_formats: u32,
    pub mutable_formats: *mut Format,
    pub buffer_formatted_write: Bool,
    pub buffer_formatted_read: Bool,
    pub runtime_resource: *mut RuntimeResource
}

impl Default for RuntimeOpCreateResourceArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpCreateResourceArgs, ffi::RpsRuntimeOpCreateResourceArgs);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpDestroyResourceArgs {
    pub resource_type: ResourceType,
    pub num_resources: u32,
    pub runtime_resources: *const RuntimeResource
}

impl Default for RuntimeOpDestroyResourceArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpDestroyResourceArgs, ffi::RpsRuntimeOpDestroyResourceArgs);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpCreateNodeUserResourcesArgs {
    pub user_context: *const c_void,
    pub args: *const *const c_void,
    pub num_args: u32,
    pub node_tag: u32
}

impl Default for RuntimeOpCreateNodeUserResourcesArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpCreateNodeUserResourcesArgs, ffi::RpsRuntimeOpCreateNodeUserResourcesArgs);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RuntimeDebugMarkerMode(u32);

impl RuntimeDebugMarkerMode {
    pub const BEGIN: Self = Self(ffi::RpsRuntimeDebugMarkerMode_RPS_RUNTIME_DEBUG_MARKER_BEGIN as _);
    pub const LABEL: Self = Self(ffi::RpsRuntimeDebugMarkerMode_RPS_RUNTIME_DEBUG_MARKER_LABEL as _);
    pub const END: Self = Self(ffi::RpsRuntimeDebugMarkerMode_RPS_RUNTIME_DEBUG_MARKER_END as _);
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct RuntimeRenderPassFlags: u32 {
        const NONE = ffi::RpsRuntimeRenderPassFlagBits_RPS_RUNTIME_RENDER_PASS_FLAG_NONE as _;
        const SUSPENDING = ffi::RpsRuntimeRenderPassFlagBits_RPS_RUNTIME_RENDER_PASS_SUSPENDING as _;
        const RESUMING = ffi::RpsRuntimeRenderPassFlagBits_RPS_RUNTIME_RENDER_PASS_RESUMING as _;
        const EXECUTE_SECONDARY_COMMAND_BUFFERS = ffi::RpsRuntimeRenderPassFlagBits_RPS_RUNTIME_RENDER_PASS_EXECUTE_SECONDARY_COMMAND_BUFFERS as _;
        const SECONDARY_COMMAND_BUFFER = ffi::RpsRuntimeRenderPassFlagBits_RPS_RUNTIME_RENDER_PASS_SECONDARY_COMMAND_BUFFER as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpRecordDebugMarkerArgs {
    pub command_buffer: RuntimeCommandBuffer,
    pub user_record_context: *const c_void,
    pub mode: RuntimeDebugMarkerMode,
    pub text: *const c_char
}

impl Default for RuntimeOpRecordDebugMarkerArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpRecordDebugMarkerArgs, ffi::RpsRuntimeOpRecordDebugMarkerArgs);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeOpSetDebugNameArgs {
    pub resource: RuntimeResource,
    pub resource_type: ResourceType,
    pub name: *const c_char
}

impl Default for RuntimeOpSetDebugNameArgs {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeOpSetDebugNameArgs, ffi::RpsRuntimeOpSetDebugNameArgs);

pub type PfnRuntimeDeviceBuildRenderGraphPhases =
    Option<unsafe extern "C" fn(user_context: *mut c_void, render_graph: RenderGraph, phase_info: *const *const RenderGraphPhaseInfo, num_phaeses: *mut u32) -> Result>;

pub type PfnRuntimeDeviceDestroy = Option<unsafe extern "C" fn(user_context: *mut c_void)>;

pub type PfnRuntimeCreateHeap = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpCreateHeapArgs) -> Result>;

pub type PfnRuntimeDestroyHeap = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpDestroyHeapArgs)>;

pub type PfnRuntimeCreateResource = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpCreateResourceArgs) -> Result>;

pub type PfnRuntimeDestroyResource = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpDestroyResourceArgs)>;

pub type PfnRuntimeOpCreateNodeUserResources = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpCreateNodeUserResourcesArgs) -> Result>;

pub type PfnRuntimeOpDestroyNodeUserResources = Option<unsafe extern "C" fn(user_context: *mut c_void)>;

pub type PfnRuntimeOpRecordDebugMarker = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpRecordDebugMarkerArgs)>;

pub type PfnRuntimeOpSetDebugName = Option<unsafe extern "C" fn(user_context: *mut c_void, args: *const RuntimeOpSetDebugNameArgs)>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimeCallbacks {
    pub pfn_build_render_graph_phases: PfnRuntimeDeviceBuildRenderGraphPhases,
    pub pfn_destroy_runtime: PfnRuntimeDeviceDestroy,
    pub pfn_create_heap: PfnRuntimeCreateHeap,
    pub pfn_destroy_heap: PfnRuntimeDestroyHeap,
    pub pfn_create_resource: PfnRuntimeCreateResource,
    pub pfn_destroy_resource: PfnRuntimeDestroyResource,
    pub pfn_create_node_resources: PfnRuntimeOpCreateNodeUserResources,
    pub pfn_destroy_node_resources: PfnRuntimeOpDestroyNodeUserResources,
    pub pfn_record_debug_marker: PfnRuntimeOpRecordDebugMarker,
    pub pfn_set_debug_name: PfnRuntimeOpSetDebugName
}

assert_size_and_align!(RuntimeCallbacks, ffi::RpsRuntimeCallbacks);
