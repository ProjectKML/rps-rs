use std::{
    ffi::{c_char, c_void},
    mem,
    mem::MaybeUninit
};

use bitflags::bitflags;

use crate::{
    core::{Result, RpsResult},
    ffi, result_from_ffi,
    utils::{assert_size_and_align, define_handle},
    AccessAttr, Bool, ClearValue, CmdRenderTargetInfo, CmdViewportInfo, Constant, Device, DeviceCreateInfo, Format, Index32, NodeDeclId, NodeId, ParamId, RandomNumberGenerator,
    ResourceDesc, ResourceId, RpslEntry, RuntimeCallbacks, RuntimeRenderPassFlags, SemanticAttr, SubresourceRange, TypeInfo, Variable, INDEX_NONE_U32
};

define_handle!(RuntimeDevice);
define_handle!(RenderGraph);
define_handle!(RenderGraphBuilder);
define_handle!(RenderGraphPhase);
define_handle!(Subprogram);
define_handle!(RuntimeHeap);
define_handle!(RuntimeResource);
define_handle!(RuntimeCommandBuffer);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ParamAttr {
    pub access: AccessAttr,
    pub semantic: SemanticAttr
}

assert_size_and_align!(ParamAttr, ffi::RpsParamAttr);

define_handle!(ParamAttrList);
define_handle!(NodeAttrList);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ScheduleFlags: u32 {
        const UNSPECIFIED = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_UNSPECIFIED as _;
        const KEEP_PROGRAM_ORDER = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_KEEP_PROGRAM_ORDER_BIT as _;
        const PREFER_MEMORY_SAVING = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_PREFER_MEMORY_SAVING_BIT as _;
        const RANDOM_ORDER = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_RANDOM_ORDER_BIT as _;
        const MINIMIZE_COMPUTE_GFX_SWITCH = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_MINIMIZE_COMPUTE_GFX_SWITCH_BIT as _;
        const DISABLE_DEAD_CODE_ELIMINATION = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_DISABLE_DEAD_CODE_ELIMINATION_BIT as _;
        const WORKLOAD_TYPE_PIPELINING_DISABLE = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_WORKLOAD_TYPE_PIPELINING_DISABLE_BIT as _;
        const WORKLOAD_TYPE_PIPELINING_AGGRESSIVE = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_WORKLOAD_TYPE_PIPELINING_AGGRESSIVE_BIT as _;
        const ALLOW_SPLIT_BARRIERS = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_ALLOW_SPLIT_BARRIERS_BIT as _;
        const AVOID_RESCHEDULE = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_AVOID_RESCHEDULE_BIT as _;
        const ALLOW_FRAME_OVERLAP = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_ALLOW_FRAME_OVERLAP_BIT as _;
        const PREFER_RENDERPASS_TRANSITIONS = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_PREFER_RENDERPASS_TRANSITIONS_BIT as _;
        const DISABLE_RENDERPASS_TRANSITIONS = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_DISABLE_RENDERPASS_TRANSITIONS_BIT as _;
        const DEFAULT = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_DEFAULT as _;
        const DEFAULT_PERFORMANCE = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_DEFAULT_PERFORMANCE as _;
        const DEFAULT_MEMORY = ffi::RpsScheduleFlagBits_RPS_SCHEDULE_DEFAULT_MEMORY as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct DiagnosticFlags: u32 {
        const NONE = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_NONE as _;
        const ENABLE_PRE_SCHEDULE_DUMP = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_PRE_SCHEDULE_DUMP as _;
        const ENABLE_POST_SCHEDULE_DUMP = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_POST_SCHEDULE_DUMP as _;
        const ENABLE_DAG_DUMP = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_DAG_DUMP as _;
        const ENABLE_SOURCE_LOCATION = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_SOURCE_LOCATION as _;
        const ENABLE_RUNTIME_DEBUG_NAMES = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_RUNTIME_DEBUG_NAMES as _;
        const ENABLE_ALL = ffi::RpsDiagnosticFlagBits_RPS_DIAGNOSTIC_ENABLE_ALL as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct RenderGraphFlags: u32 {
        const NONE = ffi::RpsRenderGraphFlagBits_RPS_RENDER_GRAPH_FLAG_NONE as _;
        const DISALLOW_UNBOUND_NODES = ffi::RpsRenderGraphFlagBits_RPS_RENDER_GRAPH_DISALLOW_UNBOUND_NODES_BIT as _;
        const NO_GPU_MEMORY_ALIASING = ffi::RpsRenderGraphFlagBits_RPS_RENDER_GRAPH_NO_GPU_MEMORY_ALIASING as _;
    }
}

pub const MAX_QUEUES: usize = 8;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct NodeDeclFlags: u32 {
        const NONE = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_FLAG_NONE as _;
        const GRAPHICS = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_GRAPHICS_BIT as _;
        const COMPUTE = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_COMPUTE_BIT as _;
        const COPY = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_COPY_BIT as _;
        const PREFER_RENDER_PASS = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_PREFER_RENDER_PASS as _;
        const PREFER_ASYNC = ffi::RpsNodeDeclFlagBits_RPS_NODE_DECL_PREFER_ASYNC as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ParameterFlags: u32 {
        const NONE = ffi::RpsParameterFlagBits_RPS_PARAMETER_FLAG_NONE as _;
        const OUT = ffi::RpsParameterFlagBits_RPS_PARAMETER_FLAG_OUT_BIT as _;
        const OPTIONAL = ffi::RpsParameterFlagBits_RPS_PARAMETER_FLAG_OPTIONAL_BIT as _;
        const RESOURCE = ffi::RpsParameterFlagBits_RPS_PARAMETER_FLAG_RESOURCE_BIT as _;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct CmdCallbackFlags: u32 {
        const NONE = ffi::RpsCmdCallbackFlagBits_RPS_CMD_CALLBACK_FLAG_NONE as _;
        const MULTI_THREADED = ffi::RpsCmdCallbackFlagBits_RPS_CMD_CALLBACK_MULTI_THREADED_BIT as _;
        const CUSTOM_RENDER_TARGETS = ffi::RpsCmdCallbackFlagBits_RPS_CMD_CALLBACK_CUSTOM_RENDER_TARGETS_BIT as _;
        const CUSTOM_VIEWPORT = ffi::RpsCmdCallbackFlagBits_RPS_CMD_CALLBACK_CUSTOM_VIEWPORT_BIT as _;
        const CUSTOM_STATE_SETUP = ffi::RpsCmdCallbackFlagBits_RPS_CMD_CALLBACK_CUSTOM_STATE_SETUP_BIT as _;
    }
}

pub type PfnCmdCallback = Option<unsafe extern "C" fn(context: *const CmdCallbackContext)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CmdCallback {
    pub pfn_callback: PfnCmdCallback,
    pub user_context: *mut c_void,
    pub flags: CmdCallbackFlags
}

impl Default for CmdCallback {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(CmdCallback, ffi::RpsCmdCallback);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ParameterDesc {
    pub type_info: TypeInfo,
    pub array_size: u32,
    pub attr: Constant,
    pub name: *const c_char,
    pub flags: ParameterFlags
}

impl Default for ParameterDesc {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(ParameterDesc, ffi::RpsParameterDesc);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NodeDesc {
    pub flags: NodeDeclFlags,
    pub num_params: u32,
    pub param_descs: *const ParameterDesc,
    pub name: *const c_char
}

impl Default for NodeDesc {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(NodeDesc, ffi::RpsNodeDesc);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphSignatureDesc {
    pub num_params: u32,
    pub num_node_descs: u32,
    pub max_external_resources: u32,
    pub param_descs: *const ParameterDesc,
    pub node_descs: *const NodeDesc,
    pub name: *const c_char
}

impl Default for RenderGraphSignatureDesc {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphSignatureDesc, ffi::RpsRenderGraphSignatureDesc);

#[inline]
pub unsafe fn cmd_callback_report_error(context: *const CmdCallbackContext, error_code: Result) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdCallbackReportError(context.cast(), mem::transmute(error_code)))
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct NodeFlags: u32 {
        const NONE = ffi::RpsNodeFlagBits_RPS_NODE_FLAG_NONE as _;
        const PREFER_ASYNC = ffi::RpsNodeFlagBits_RPS_NODE_PREFER_ASYNC as _;
    }
}

pub type PfnRenderGraphBuild = Option<unsafe extern "C" fn(builder: RenderGraphBuilder, args: *const Constant, num_args: u32)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphUpdateInfo {
    pub frame_index: u64,
    pub gpu_completed_frame_index: u64,
    pub schedule_flags: ScheduleFlags,
    pub diagnostic_flags: DiagnosticFlags,
    pub num_args: u32,
    pub args: *const Constant,
    pub arg_resources: *const *const RuntimeResource,
    pub pfn_build_callback: PfnRenderGraphBuild,
    pub random_number_generator: *const RandomNumberGenerator
}

impl Default for RenderGraphUpdateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphUpdateInfo, ffi::RpsRenderGraphUpdateInfo);

pub const MAX_QUEUED_FRAMES: usize = 16;
pub const GPU_COMPLETED_FRAME_INDEX_NONE: u64 = u64::MAX;

pub type PfnRenderGraphPhaseRun = Option<unsafe extern "C" fn(render_graph: RenderGraph, update_info: *const RenderGraphUpdateInfo, phase: RenderGraphPhase)>;
pub type PfnRenderGraphPhaseDestroy = Option<unsafe extern "C" fn(phase: RenderGraphPhase)>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderGraphPhaseInfo {
    pub phase: RenderGraphPhase,
    pub pfn_run: PfnRenderGraphPhaseRun,
    pub pfn_destroy: PfnRenderGraphPhaseDestroy
}

assert_size_and_align!(RenderGraphPhaseInfo, ffi::RpsRenderGraphPhaseInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryTypeInfo {
    pub default_heap_size: u64,
    pub min_alignment: u32
}

assert_size_and_align!(MemoryTypeInfo, ffi::RpsMemoryTypeInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuMemoryRequirement {
    pub size: u64,
    pub alignment: u32,
    pub memory_type_index: Index32
}

assert_size_and_align!(GpuMemoryRequirement, ffi::RpsGpuMemoryRequirement);

pub type HeapId = Index32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HeapPlacement {
    pub heap_id: HeapId,
    pub offset: u64
}

assert_size_and_align!(HeapPlacement, ffi::RpsHeapPlacement);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RuntimeResourceInfo {
    pub resource: RuntimeResource,
    pub resource_desc: ResourceDesc,
    pub num_subresources: u32,
    pub full_range: SubresourceRange,
    pub heap_id: HeapId,
    pub alloc_info: GpuMemoryRequirement
}

assert_size_and_align!(RuntimeResourceInfo, ffi::RpsRuntimeResourceInfo);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct QueueFlags: u32 {
        const NONE = ffi::RpsQueueFlagBits_RPS_QUEUE_FLAG_NONE as _;
        const GRAPHICS = ffi::RpsQueueFlagBits_RPS_QUEUE_FLAG_GRAPHICS as _;
        const COMPUTE = ffi::RpsQueueFlagBits_RPS_QUEUE_FLAG_COMPUTE as _;
        const COPY = ffi::RpsQueueFlagBits_RPS_QUEUE_FLAG_COPY as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeDeviceCreateInfo {
    pub user_context: *mut c_void,
    pub callbacks: RuntimeCallbacks
}

impl Default for RuntimeDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RuntimeDeviceCreateInfo, ffi::RpsRuntimeDeviceCreateInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NullRuntimeDeviceCreateInfo {
    pub device_create_info: *const DeviceCreateInfo,
    pub runtime_create_info: *const RuntimeDeviceCreateInfo
}

impl Default for NullRuntimeDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(NullRuntimeDeviceCreateInfo, ffi::RpsNullRuntimeDeviceCreateInfo);

#[inline]
pub unsafe fn null_runtime_device_create(create_info: *const DeviceCreateInfo) -> RpsResult<Device> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsNullRuntimeDeviceCreate(create_info.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramCreateInfo {
    pub signature_desc: *const RenderGraphSignatureDesc,
    pub rpsl_entry_point: RpslEntry,
    pub default_node_callback: CmdCallback
}

impl Default for ProgramCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(ProgramCreateInfo, ffi::RpsProgramCreateInfo);

#[inline]
pub unsafe fn rpsl_entry_get_signature_desc(rpsl_entry: RpslEntry) -> RpsResult<RenderGraphSignatureDesc> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsRpslEntryGetSignatureDesc(rpsl_entry.into_raw().cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn program_create(device: Device, create_info: *const ProgramCreateInfo) -> RpsResult<Subprogram> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsProgramCreate(device.into_raw().cast(), create_info.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn program_destroy(program: Subprogram) {
    ffi::rpsProgramDestroy(program.into_raw().cast());
}

#[inline]
pub unsafe fn program_bind_node_callback(program: Subprogram, name: *const c_char, callback: *const CmdCallback) -> RpsResult<()> {
    result_from_ffi(ffi::rpsProgramBindNodeCallback(program.into_raw().cast(), name, callback.cast()))
}

#[inline]
pub unsafe fn program_bind_node_subprogram(program: Subprogram, name: *const c_char, subprogram: Subprogram) -> RpsResult<()> {
    result_from_ffi(ffi::rpsProgramBindNodeSubprogram(program.into_raw().cast(), name, subprogram.into_raw().cast()))
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphCreateScheduleInfo {
    pub schedule_flags: ScheduleFlags,
    pub num_queues: u32,
    pub queue_infos: *const QueueFlags
}

impl Default for RenderGraphCreateScheduleInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphCreateMemoryInfo {
    pub num_heaps: u32,
    pub heap_budget_mibs: *const u32
}

impl Default for RenderGraphCreateMemoryInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphCreateInfo {
    pub schedule_info: RenderGraphCreateScheduleInfo,
    pub memory_info: RenderGraphCreateMemoryInfo,
    pub main_entry_create_info: ProgramCreateInfo,
    pub render_graph_flags: RenderGraphFlags,
    pub num_phases: u32,
    pub phases: *const RenderGraphPhaseInfo
}

impl Default for RenderGraphCreateInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphCreateInfo, ffi::RpsRenderGraphCreateInfo);

#[inline]
pub unsafe fn render_graph_create(device: Device, create_info: *const RenderGraphCreateInfo) -> RpsResult<RenderGraph> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsRenderGraphCreate(device.into_raw().cast(), create_info.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn render_graph_update(render_graph: RenderGraph, update_info: *const RenderGraphUpdateInfo) -> RpsResult<()> {
    result_from_ffi(ffi::rpsRenderGraphUpdate(render_graph.into_raw().cast(), update_info.cast()))
}

#[inline]
pub unsafe fn render_graph_destroy(render_graph: RenderGraph) {
    ffi::rpsRenderGraphDestroy(render_graph.into_raw().cast());
}

#[inline]
pub unsafe fn render_graph_allocate_data(render_graph_builder: RenderGraphBuilder, size: usize) -> *const c_void {
    ffi::rpsRenderGraphAllocateData(render_graph_builder.into_raw().cast(), size)
}

#[inline]
pub unsafe fn render_graph_allocate_data_aligned(render_graph_builder: RenderGraphBuilder, size: usize, alignment: usize) -> *const c_void {
    ffi::rpsRenderGraphAllocateDataAligned(render_graph_builder.into_raw().cast(), size, alignment)
}

#[inline]
pub unsafe fn render_graph_declare_dynamic_node(render_graph_builder: RenderGraphBuilder, node_desc: *const NodeDesc) -> NodeDeclId {
    ffi::rpsRenderGraphDeclareDynamicNode(render_graph_builder.into_raw().cast(), node_desc.cast())
}

#[inline]
pub unsafe fn render_graph_get_param_variable(render_graph_builder: RenderGraphBuilder, param_id: ParamId) -> Variable {
    ffi::rpsRenderGraphGetParamVariable(render_graph_builder.into_raw().cast(), param_id)
}

#[inline]
pub unsafe fn render_graph_get_param_resource_id(render_graph_builder: RenderGraphBuilder, param_id: ParamId) -> ResourceId {
    ffi::rpsRenderGraphGetParamResourceId(render_graph_builder.into_raw().cast(), param_id)
}

#[inline]
pub unsafe fn render_graph_declare_resource(render_graph_builder: RenderGraphBuilder, name: *const c_char, local_id: ResourceId, arg: Variable) -> ResourceId {
    ffi::rpsRenderGraphDeclareResource(render_graph_builder.into_raw().cast(), name, local_id, arg)
}

#[inline]
pub unsafe fn render_graph_add_node(
    render_graph_builder: RenderGraphBuilder,
    node_decl_id: NodeDeclId,
    user_tag: u32,
    callback: PfnCmdCallback,
    callback_user_context: *mut c_void,
    args: *const Variable,
    num_args: u32
) -> NodeId {
    ffi::rpsRenderGraphAddNode(
        render_graph_builder.into_raw().cast(),
        node_decl_id,
        user_tag,
        mem::transmute(callback),
        callback_user_context,
        args,
        num_args
    )
}

#[inline]
pub unsafe fn render_graph_get_resource_info(render_graph_builder: RenderGraphBuilder, resource_id: ResourceId, temporal_layer_index: u32) -> RpsResult<RuntimeResourceInfo> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsRenderGraphGetResourceInfo(
        render_graph_builder.into_raw().cast(),
        resource_id,
        temporal_layer_index,
        &mut result as *mut _ as *mut _
    ))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn render_graph_get_output_parameter_resource_infos(
    render_graph_builder: RenderGraphBuilder,
    param_id: ParamId,
    array_offset: u32,
    num_resources: u32,
    resource_infos: *mut RuntimeResourceInfo
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsRenderGraphGetOutputParameterResourceInfos(
        render_graph_builder.into_raw().cast(),
        param_id,
        array_offset,
        num_resources,
        resource_infos.cast()
    ))
}

#[inline]
pub unsafe fn render_graph_get_main_entry(render_graph: RenderGraph) -> Subprogram {
    Subprogram(ffi::rpsRenderGraphGetMainEntry(render_graph.into_raw().cast()).cast())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandBatch {
    pub queue_index: u32,
    pub wait_fences_begin: u32,
    pub num_wait_fences: u32,
    pub signal_fence_index: u32,
    pub cmd_begin: u32,
    pub num_cmds: u32
}

assert_size_and_align!(CommandBatch, ffi::RpsCommandBatch);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphBatchLayout {
    pub num_cmd_batches: u32,
    pub num_fence_signals: u32,
    pub cmd_batches: *const CommandBatch,
    pub wait_fence_indices: *const u32
}

impl Default for RenderGraphBatchLayout {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphBatchLayout, ffi::RpsRenderGraphBatchLayout);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct RecordCommandFlags: u32 {
        const NONE = ffi::RpsRecordCommandFlagBits_RPS_RECORD_COMMAND_FLAG_NONE as _;
        const ENABLE_COMMAND_DEBUG_MARKERS = ffi::RpsRecordCommandFlagBits_RPS_RECORD_COMMAND_FLAG_ENABLE_COMMAND_DEBUG_MARKERS as _;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderGraphRecordCommandInfo {
    pub cmd_buffer: RuntimeCommandBuffer,
    pub user_context: *mut c_void,
    pub frame_index: u64,
    pub cmd_begin_index: u32,
    pub num_cmds: u32,
    pub flags: RecordCommandFlags
}

impl Default for RenderGraphRecordCommandInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphRecordCommandInfo, ffi::RpsRenderGraphRecordCommandInfo);

#[inline]
pub unsafe fn render_graph_get_batch_layout(render_graph: RenderGraph) -> RpsResult<RenderGraphBatchLayout> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsRenderGraphGetBatchLayout(render_graph.into_raw().cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn render_graph_record_commands(render_graph: RenderGraph, record_info: *const RenderGraphRecordCommandInfo) -> RpsResult<()> {
    result_from_ffi(ffi::rpsRenderGraphRecordCommands(render_graph.into_raw().cast(), record_info.cast()))
}

pub const CMD_ID_INVALID: u32 = INDEX_NONE_U32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CmdDiagnosticInfoTransitionCmd {
    pub dummy: u32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CmdDiagnosticInfoTransitionTransition {
    pub prev_access: AccessAttr,
    pub next_access: AccessAttr,
    pub range: SubresourceRange,
    pub resource_index: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CmdDiagnosticInfoTransition {
    pub cmd: CmdDiagnosticInfoTransitionCmd,
    pub transition: CmdDiagnosticInfoTransitionTransition
}

impl Default for CmdDiagnosticInfoTransition {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CmdDiagnosticInfo {
    pub cmd_index: u32,
    pub is_transition: Bool,
    pub transition: CmdDiagnosticInfoTransition
}

assert_size_and_align!(CmdDiagnosticInfo, ffi::RpsCmdDiagnosticInfo);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ResourceDiagnosticInfo {
    pub name: *const c_char,
    pub temporal_child_index: u32,
    pub is_external: Bool,
    pub desc: ResourceDesc,
    pub clear_value: ClearValue,
    pub all_accesses: AccessAttr,
    pub initial_access: AccessAttr,
    pub lifetime_begin: u32,
    pub lifetime_end: u32,
    pub alloc_requirement: GpuMemoryRequirement,
    pub alloc_placement: HeapPlacement,
    pub runtime_resource: RuntimeResource
}

impl Default for ResourceDiagnosticInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(ResourceDiagnosticInfo, ffi::RpsResourceDiagnosticInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HeapDiagnosticInfo {
    pub size: u64,
    pub used_size: u64,
    pub max_used_size: u64,
    pub alignment: u32,
    pub memory_type_index: u32,
    pub runtime_heap: RuntimeHeap
}

assert_size_and_align!(HeapDiagnosticInfo, ffi::RpsHeapDiagnosticInfo);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphDiagnosticInfo {
    pub num_resource_infos: u32,
    pub num_command_infos: u32,
    pub num_heap_infos: u32,
    pub resource_diag_infos: *const ResourceDiagnosticInfo,
    pub cmd_diag_infos: *const CmdDiagnosticInfo,
    pub heap_diag_infos: *const HeapDiagnosticInfo
}

impl Default for RenderGraphDiagnosticInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct RenderGraphDiagnosticInfoFlags: u32 {
        const DEFAULT = ffi::RpsRenderGraphDiagnosticInfoFlagBits_RPS_RENDER_GRAPH_DIAGNOSTIC_INFO_DEFAULT as _;
        const USE_CACHED_BIT = ffi::RpsRenderGraphDiagnosticInfoFlagBits_RPS_RENDER_GRAPH_DIAGNOSTIC_INFO_USE_CACHED_BIT as _;
    }
}

#[inline]
pub unsafe fn render_graph_get_diagnostics_info(render_graph: RenderGraph, diagnostic_flags: RenderGraphDiagnosticInfoFlags) -> RpsResult<RenderGraphDiagnosticInfo> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsRenderGraphGetDiagnosticInfo(
        render_graph.into_raw().cast(),
        &mut result as *mut _ as *mut _,
        mem::transmute(diagnostic_flags)
    ))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CmdCallbackContext {
    pub command_buffer: RuntimeCommandBuffer,
    pub user_record_context: *mut c_void,
    pub cmd_callback_context: *mut c_void,
    pub args: *const *mut c_void,
    pub num_args: u32,
    pub user_tag: u32
}

impl Default for CmdCallbackContext {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(CmdCallbackContext, ffi::RpsCmdCallbackContext);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceAccessInfo {
    pub resource_id: ResourceId,
    pub range: SubresourceRange,
    pub access: AccessAttr,
    pub view_format: Format
}

assert_size_and_align!(ResourceAccessInfo, ffi::RpsResourceAccessInfo);

#[inline]
pub unsafe fn cmd_get_render_targets_info(context: *const CmdCallbackContext) -> RpsResult<CmdRenderTargetInfo> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdGetRenderTargetsInfo(context.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn cmd_get_viewport_info(context: *const CmdCallbackContext) -> RpsResult<CmdViewportInfo> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdGetViewportInfo(context.cast(), &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CmdRenderPassBeginInfo {
    pub flags: RuntimeRenderPassFlags
}

#[inline]
pub unsafe fn cmd_clone_context(context: *const CmdCallbackContext, cmd_buffer_for_derived_context: RuntimeCommandBuffer) -> RpsResult<*const CmdCallbackContext> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdCloneContext(
        context.cast(),
        mem::transmute(cmd_buffer_for_derived_context),
        &mut result as *mut _ as *mut _
    ))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn cmd_begin_render_pass(context: *const CmdCallbackContext, flags: RuntimeRenderPassFlags) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdBeginRenderPass(context.cast(), mem::transmute(flags)))
}

#[inline]
pub unsafe fn cmd_end_render_pass(context: *const CmdCallbackContext) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdEndRenderPass(context.cast()))
}

#[inline]
pub unsafe fn cmd_set_command_buffer(context: *const CmdCallbackContext, cmd_buffer: RuntimeCommandBuffer) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdSetCommandBuffer(context.cast(), mem::transmute(cmd_buffer)))
}

#[inline]
pub unsafe fn cmd_get_node_name(context: *const CmdCallbackContext, node_names: *mut *const c_char, node_name_length: *mut usize) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdGetNodeName(context.cast(), node_names, node_name_length))
}

#[inline]
pub unsafe fn cmd_get_param_desc(context: *const CmdCallbackContext, param_id: ParamId) -> RpsResult<ParameterDesc> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdGetParamDesc(context.cast(), param_id, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn cmd_get_arg_resource_desc_array(
    context: *const CmdCallbackContext,
    arg_index: ParamId,
    src_array_offset: u32,
    resource_descs: *mut ResourceDesc,
    num_descs: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdGetArgResourceDescArray(context.cast(), arg_index, src_array_offset, resource_descs.cast(), num_descs))
}

#[inline]
pub unsafe fn cmd_get_arg_resource_desc(context: *const CmdCallbackContext, arg_index: ParamId) -> RpsResult<ResourceDesc> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdGetArgResourceDesc(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn cmd_get_arg_resource_access_info_array(
    context: *const CmdCallbackContext,
    arg_index: ParamId,
    src_array_offset: u32,
    resource_access_infos: *mut ResourceAccessInfo,
    num_accessess: u32
) -> RpsResult<()> {
    result_from_ffi(ffi::rpsCmdGetArgResourceAccessInfoArray(
        context.cast(),
        arg_index,
        src_array_offset,
        resource_access_infos.cast(),
        num_accessess
    ))
}

#[inline]
pub unsafe fn cmd_get_arg_resource_access_info(context: *const CmdCallbackContext, arg_index: ParamId) -> RpsResult<ResourceAccessInfo> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsCmdGetArgResourceAccessInfo(context.cast(), arg_index, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn cmd_get_arg(context: *const CmdCallbackContext, arg_index: u32) -> Variable {
    *(*context).args.offset(arg_index as _)
}

pub type PfnAcquireRuntimeCommandBuffer =
    Option<unsafe extern "C" fn(user_context: *mut c_void, queue_index: u32, num_cmd_buffers: u32, cmd_buffesr: *mut RuntimeCommandBuffer, cmd_buffer_identifiers: *mut u32)>;
pub type PfnSubmitRuntimeCommandBuffer =
    Option<unsafe extern "C" fn(user_context: *mut c_void, queue_index: u32, runtime_cmd_bufs: *const RuntimeCommandBuffer, num_runtime_cmd_bufs: u32, wait_id: u32, signal_id: u32)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderGraphExecuteInfo {
    pub user_context: *mut c_void,
    pub pfn_acquire_runtime_cmd_buf_cb: PfnAcquireRuntimeCommandBuffer,
    pub pfn_submit_runtime_cmd_buf_cb: PfnSubmitRuntimeCommandBuffer
}

impl Default for RenderGraphExecuteInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RenderGraphExecuteInfo, ffi::RpsRenderGraphExecuteInfo);

#[inline]
pub unsafe fn render_graph_execute(render_graph: RenderGraph, execute_info: *const RenderGraphExecuteInfo) -> RpsResult<()> {
    result_from_ffi(ffi::rpsRenderGraphExecute(render_graph.into_raw().cast(), execute_info.cast()))
}
