use std::{
    ffi::{c_char, c_void},
    mem,
    mem::MaybeUninit
};

use bitflags::bitflags;

use crate::{
    ffi, result_from_ffi,
    utils::{assert_size_and_align, define_handle},
    RpsResult
};

pub type Bool = i32;

pub const TRUE: Bool = 1;
pub const FALSE: Bool = 0;

pub type Flags16 = u16;
pub type Flags32 = u32;
pub type Flags64 = u64;
pub type Index32 = u32;

pub const INDEX_NONE_U32: u32 = u32::MAX;
pub const INDEX_NONE_I32: i32 = -1;
pub const NAME_MAX_LEN: usize = 256;

pub type PfnAlloc = Option<unsafe extern "C" fn(*mut c_void, usize, usize)>;
pub type PfnRealloc = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize, usize, usize)>;
pub type PfnFree = Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Allocator {
    pub pfn_alloc: PfnAlloc,
    pub pfn_realloc: PfnRealloc,
    pub pfn_free: PfnFree,
    pub context: *mut c_void
}

impl Default for Allocator {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(Allocator, ffi::RpsAllocator);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AllocInfo {
    pub size: usize,
    pub alignment: usize
}

assert_size_and_align!(AllocInfo, ffi::RpsAllocInfo);

pub type PfnPrintf = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type PfnVPrintf = Option<unsafe extern "C" fn(*mut c_void, *const c_char)>; //TODO:

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Printer {
    pub pfn_printf: PfnPrintf,
    pub pfn_vprintf: PfnVPrintf,
    pub context: *mut c_void
}

impl Default for Printer {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(Printer, ffi::RpsPrinter);

pub type PfnRandomUniformInt = Option<unsafe extern "C" fn(*mut c_void, i32, i32)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RandomNumberGenerator {
    pub pfn_random_uniform_int: PfnRandomUniformInt,
    pub context: *mut c_void
}

impl Default for RandomNumberGenerator {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(RandomNumberGenerator, ffi::RpsRandomNumberGenerator);

define_handle!(Device);

pub type PfnDeviceOnDestroy = Option<unsafe extern "C" fn(Device)>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceCreateInfo {
    pub allocator: Allocator,
    pub printer: Printer,
    pub private_data_alloc_info: AllocInfo,
    pub pfn_device_on_destroy: PfnDeviceOnDestroy
}

assert_size_and_align!(DeviceCreateInfo, ffi::RpsDeviceCreateInfo);

#[inline]
pub unsafe fn device_create(create_info: &DeviceCreateInfo) -> RpsResult<Device> {
    let mut result = MaybeUninit::uninit();
    result_from_ffi(ffi::rpsDeviceCreate(create_info as *const DeviceCreateInfo as _, &mut result as *mut _ as *mut _))?;
    Ok(result.assume_init())
}

#[inline]
pub unsafe fn device_destroy(device: Device) {
    ffi::rpsDeviceDestroy(device.into_raw() as _);
}

#[inline]
pub unsafe fn device_get_private_data(device: Device) -> *const c_void {
    ffi::rpsDeviceGetPrivateData(device.into_raw() as _)
}

#[inline]
pub unsafe fn set_global_debug_printer(printer: *const Printer) {
    ffi::rpsSetGlobalDebugPrinter(printer.cast());
}

#[inline]
pub unsafe fn get_global_debug_printer() -> *const Printer {
    ffi::rpsGetGlobalDebugPrinter().cast()
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TypeInfo {
    pub size: u16,
    pub id: u16
}

assert_size_and_align!(TypeInfo, ffi::RpsTypeInfo);

impl TypeInfo {
    #[inline]
    pub fn init_from_size(size: usize) -> Self {
        Self {
            size: size as _,
            id: ffi::RpsBuiltInTypeIds_RPS_TYPE_OPAQUE as _
        }
    }

    #[inline]
    pub fn init_from_size_and_type_id(size: usize, type_id: TypeId) -> Self {
        Self { size: size as _, id: type_id as _ }
    }

    #[inline]
    pub fn init_from_type<T>() -> Self {
        Self::init_from_size(mem::size_of::<T>())
    }

    #[inline]
    pub fn init_from_type_and_id<T>(type_id: TypeId) -> Self {
        Self::init_from_size_and_type_id(mem::size_of::<T>(), type_id)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BuiltInTypeIds {
    #[default]
    Bool = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_BOOL as _,
    Int8 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT8 as _,
    UInt8 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT8 as _,
    Int16 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT16 as _,
    UInt16 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT16 as _,
    Int32 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT32 as _,
    UInt32 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT32 as _,
    Int64 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT64 as _,
    UInt64 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT64 as _,
    Float32 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_FLOAT32 as _,
    Float64 = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_FLOAT64 as _,
    MaxValue = ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_MAX_VALUE as _,
    RuntimeDefinedBegin = ffi::RpsBuiltInTypeIds_RPS_TYPE_RUNTIME_DEFINED_BEGIN as _,
    UserDefinedBegin = ffi::RpsBuiltInTypeIds_RPS_TYPE_USER_DEFINED_BEGIN as _
}

pub type TypeId = i32;

pub type NodeDeclId = u32;
pub type ParamId = u32;
pub type NodeId = u32;

pub const NODEDECL_ID_INVALID: u32 = INDEX_NONE_U32;
pub const PARAM_ID_INVALID: u32 = INDEX_NONE_U32;

pub type Variable = *mut c_void;
pub type Constant = *const c_void;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SubgraphFlags: u32 {
        const NONE = ffi::RpsSubgraphFlagBits_RPS_SUBGRAPH_FLAG_NONE as _;
        const ATOMIC = ffi::RpsSubgraphFlagBits_RPS_SUBGRAPH_FLAG_ATOMIC as _;
        const SEQUENTIAL = ffi::RpsSubgraphFlagBits_RPS_SUBGRAPH_FLAG_SEQUENTIAL as _;
    }
}

pub type SourceFileId = Flags32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceLocation {
    pub file: SourceFileId,
    pub line: u32
}

assert_size_and_align!(SourceLocation, ffi::RpsSourceLocation);

define_handle!(DebugInfo);

pub type RpslEntryCallFlags = Flags32;

pub type PfnRpslEntry = Option<unsafe extern "C" fn(u32, *const *const c_void, RpslEntryCallFlags)>;

define_handle!(RpslEntry);

//TODO: macros

pub type PfnRpslDynLibInit = ffi::PFN_rpslDynLibInit;

#[inline]
pub unsafe fn rpsl_dynamic_library_init(pfn_dyn_lib_init: PfnRpslDynLibInit) -> RpsResult<()> {
    result_from_ffi(ffi::rpsRpslDynamicLibraryInit(pfn_dyn_lib_init))
}

#[inline]
pub unsafe fn make_rpsl_entry_name(buf: *mut c_char, buf_size: usize, module_name: *const c_char, entry_name: *const c_char) -> *const c_char {
    ffi::rpsMakeRpslEntryName(buf, buf_size, module_name, entry_name)
}

define_handle!(JITModule);

pub type PfnJITStartup = Option<unsafe extern "C" fn(i32, *const *const c_char) -> i32>;
pub type PfnJITShutdown = Option<unsafe extern "C" fn()>;
pub type PfnJITLoad = Option<unsafe extern "C" fn(*const c_char, *mut JITModule) -> i32>;
pub type PfnJITUnload = Option<unsafe extern "C" fn(JITModule)>;
pub type PfnJITGetEntryPoint = Option<unsafe extern "C" fn(JITModule, *const c_char, *mut u64) -> i32>;

//TODO: CONSTANTS
