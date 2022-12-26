use std::{
    ffi::{c_char, c_void},
    mem,
    mem::MaybeUninit,
    path::{Path, PathBuf}
};

use bitflags::bitflags;
use libloading::Library;

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

pub type PfnAlloc = Option<unsafe extern "C" fn(*mut c_void, usize, usize) -> *mut c_void>;
pub type PfnRealloc = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize, usize, usize) -> *mut c_void>;
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

pub type VaList = ffi::va_list;

pub type PfnPrintf = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>;
pub type PfnVPrintf = Option<unsafe extern "C" fn(*mut c_void, *const c_char, VaList)>;

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

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BuiltInTypeIds(u32);

impl BuiltInTypeIds {
    pub const BOOL: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_BOOL);
    pub const INT8: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT8);
    pub const UINT8: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT8);
    pub const INT16: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT16);
    pub const UINT16: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT16);
    pub const INT32: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT32);
    pub const UINT32: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT32);
    pub const INT64: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_INT64);
    pub const UINT64: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_UINT64);
    pub const FLOAT32: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_FLOAT32);
    pub const FLOAT64: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_FLOAT64);
    pub const MAX_VALUE: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_BUILT_IN_MAX_VALUE);
    pub const RUNTIME_DEFINED_BEGIN: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_RUNTIME_DEFINED_BEGIN);
    pub const USER_DEFINED_BEGIN: Self = Self(ffi::RpsBuiltInTypeIds_RPS_TYPE_USER_DEFINED_BEGIN);
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

pub type PfnJITStartup = unsafe extern "C" fn(i32, *const *const c_char) -> i32;
pub type PfnJITShutdown = unsafe extern "C" fn();
pub type PfnJITLoad = unsafe extern "C" fn(*const c_char, *mut JITModule) -> i32;
pub type PfnJITUnload = unsafe extern "C" fn(JITModule);
pub type PfnJITGetEntryPoint = unsafe extern "C" fn(JITModule, *const c_char, *mut u64) -> i32;

pub const JIT_PROC_NAME_STARTUP: &'static [u8] = b"RpsJITStartup\0";
pub const JIT_PROC_NAME_SHUTDOWN: &'static [u8] = b"RpsJITShutdown\0";
pub const JIT_PROC_NAME_LOAD: &'static [u8] = b"RpsJITLoad\0";
pub const JIT_PROC_NAME_UNLOAD: &'static [u8] = b"RpsJITUnload\0";
pub const JIT_PROC_NAME_GETENTRYPOINT: &'static [u8] = b"RpsJITGetEntryPoint\0";

#[cfg(target_os = "windows")]
fn jit_lib_name() -> &'static Path {
    Path::new("rps-jit.dll")
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn jit_lib_name() -> &'static Path {
    todo!("Not supported at the moment")
}

#[cfg(target_os = "macos")]
fn jit_lib_name() -> &'static Path {
    todo!("Not supported at the moment")
}

pub struct JITLibrary {
    _library: Library,

    jit_startup: PfnJITStartup,
    jit_shutdown: PfnJITShutdown,
    jit_load: PfnJITLoad,
    jit_unload: PfnJITUnload,
    jit_get_entry_point: PfnJITGetEntryPoint
}

impl JITLibrary {
    pub fn new(path: Option<PathBuf>) -> Result<Self, libloading::Error> {
        let total_path = if let Some(total_path) = path {
            if total_path.is_file() {
                total_path
            } else {
                total_path.join(jit_lib_name())
            }
        } else {
            jit_lib_name().to_owned()
        };

        let library = unsafe { Library::new(&total_path) }?;

        unsafe {
            let jit_startup = mem::transmute(library.get::<PfnJITStartup>(JIT_PROC_NAME_STARTUP)?.into_raw().into_raw());
            let jit_shutdown = mem::transmute(library.get::<PfnJITShutdown>(JIT_PROC_NAME_SHUTDOWN)?.into_raw().into_raw());
            let jit_load = mem::transmute(library.get::<PfnJITLoad>(JIT_PROC_NAME_LOAD)?.into_raw().into_raw());
            let jit_unload = mem::transmute(library.get::<PfnJITUnload>(JIT_PROC_NAME_UNLOAD)?.into_raw().into_raw());
            let jit_get_entry_point = mem::transmute(library.get::<PfnJITGetEntryPoint>(JIT_PROC_NAME_GETENTRYPOINT)?.into_raw().into_raw());

            Ok(Self {
                _library: library,

                jit_startup,
                jit_shutdown,
                jit_load,
                jit_unload,
                jit_get_entry_point
            })
        }
    }

    #[inline]
    pub unsafe fn startup(&self, argc: i32, args: *const *const c_char) -> i32 {
        (self.jit_startup)(argc, args)
    }

    #[inline]
    pub unsafe fn shutdown(&self) {
        (self.jit_shutdown)();
    }

    #[inline]
    pub unsafe fn load(&self, name: *const c_char) -> RpsResult<JITModule> {
        let mut result = MaybeUninit::uninit();
        result_from_ffi((self.jit_load)(name, &mut result as *mut _ as *mut _))?;
        Ok(result.assume_init())
    }

    #[inline]
    pub unsafe fn unload(&self, jit_module: JITModule) {
        (self.jit_unload)(jit_module);
    }

    #[inline]
    pub unsafe fn get_entry_point(&self, jit_module: JITModule, symbol_name: *const c_char) -> RpsResult<u64> {
        let mut result = MaybeUninit::uninit();
        result_from_ffi((self.jit_get_entry_point)(jit_module, symbol_name, &mut result as *mut _ as *mut _))?;
        Ok(result.assume_init())
    }
}
