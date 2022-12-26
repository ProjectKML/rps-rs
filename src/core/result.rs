use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    mem, slice
};

use crate::ffi;

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Result(i32);

impl Result {
    pub const OK: Self = Self(ffi::RpsResult_RPS_OK);
    pub const UNSPECIFIED: Self = Self(ffi::RpsResult_RPS_ERROR_UNSPECIFIED);
    pub const UNRECOGNIZED_COMMAND: Self = Self(ffi::RpsResult_RPS_ERROR_UNRECOGNIZED_COMMAND);
    pub const INVALID_ARGUMENTS: Self = Self(ffi::RpsResult_RPS_ERROR_INVALID_ARGUMENTS);
    pub const INVALID_DATA: Self = Self(ffi::RpsResult_RPS_ERROR_INVALID_DATA);
    pub const INVALID_OPERATION: Self = Self(ffi::RpsResult_RPS_ERROR_INVALID_OPERATION);
    pub const OUT_OF_MEMORY: Self = Self(ffi::RpsResult_RPS_ERROR_OUT_OF_MEMORY);
    pub const FILE_NOT_FOUND: Self = Self(ffi::RpsResult_RPS_ERROR_FILE_NOT_FOUND);
    pub const INVALID_FILE_FORMAT: Self = Self(ffi::RpsResult_RPS_ERROR_INVALID_FILE_FORMAT);
    pub const UNSUPPORTED_VERSION_TOO_OLD: Self = Self(ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_OLD);
    pub const UNSUPPORTED_VERSION_TOO_NEW: Self = Self(ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_NEW);
    pub const UNKNOWN_NODE: Self = Self(ffi::RpsResult_RPS_ERROR_UNKNOWN_NODE);
    pub const INDEX_OUT_OF_BOUNDS: Self = Self(ffi::RpsResult_RPS_ERROR_INDEX_OUT_OF_BOUNDS);
    pub const COMMAND_ALREADY_FINAL: Self = Self(ffi::RpsResult_RPS_ERROR_COMMAND_ALREADY_FINAL);
    pub const INTEROP_DATA_LAYOUT_MISMATCH: Self = Self(ffi::RpsResult_RPS_ERROR_INTEROP_DATA_LAYOUT_MISMATCH);
    pub const KEY_NOT_FOUND: Self = Self(ffi::RpsResult_RPS_ERROR_KEY_NOT_FOUND);
    pub const KEY_DUPLICATED: Self = Self(ffi::RpsResult_RPS_ERROR_KEY_DUPLICATED);
    pub const NOT_IMPLEMENTED: Self = Self(ffi::RpsResult_RPS_ERROR_NOT_IMPLEMENTED);
    pub const INTEGER_OVERFLOW: Self = Self(ffi::RpsResult_RPS_ERROR_INTEGER_OVERFLOW);
    pub const RANGE_OVERLAPPING: Self = Self(ffi::RpsResult_RPS_ERROR_RANGE_OVERLAPPING);
    pub const VALIDATION_FAILED: Self = Self(ffi::RpsResult_RPS_ERROR_VALIDATION_FAILED);
    pub const INVALID_PROGRAM: Self = Self(ffi::RpsResult_RPS_ERROR_INVALID_PROGRAM);
    pub const UNSUPPORTED_MODULE_VERSION: Self = Self(ffi::RpsResult_RPS_ERROR_UNSUPPORTED_MODULE_VERSION);
    pub const TYPE_MISMATCH: Self = Self(ffi::RpsResult_RPS_ERROR_TYPE_MISMATCH);
    pub const NOT_SUPPORTED: Self = Self(ffi::RpsResult_RPS_ERROR_NOT_SUPPORTED);
    pub const RUNTIME_API_ERROR: Self = Self(ffi::RpsResult_RPS_ERROR_RUNTIME_API_ERROR);
    pub const INTERNAL_ERROR: Self = Self(ffi::RpsResult_RPS_ERROR_INTERNAL_ERROR);
    pub const CODE_COUNT: Self = Self(ffi::RpsResult_RPS_RESULT_CODE_COUNT);
}

impl Debug for Result {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = unsafe { ffi::rpsResultGetName(mem::transmute(*self)) };
        write!(f, "{}", unsafe { std::str::from_utf8_unchecked(slice::from_raw_parts(name.cast(), libc::strlen(name))) })
    }
}

impl Display for Result {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for Result {}

pub type RpsResult<T> = std::result::Result<T, Result>;

#[inline]
pub unsafe fn result_from_ffi(result: ffi::RpsResult) -> RpsResult<()> {
    let result: Result = mem::transmute(result);
    match result {
        Result::OK => Ok(()),
        _ => Err(result)
    }
}
