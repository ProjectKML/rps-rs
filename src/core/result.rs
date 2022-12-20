use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    mem
};

use crate::ffi;

#[repr(C)]
#[derive(Debug)]
pub enum Result {
    Ok = ffi::RpsResult_RPS_OK as _,
    Unspecified = ffi::RpsResult_RPS_ERROR_UNSPECIFIED as _,
    UnrecognizedCommand = ffi::RpsResult_RPS_ERROR_UNRECOGNIZED_COMMAND as _,
    InvalidArguments = ffi::RpsResult_RPS_ERROR_INVALID_ARGUMENTS as _,
    InvalidData = ffi::RpsResult_RPS_ERROR_INVALID_DATA as _,
    InvalidOperation = ffi::RpsResult_RPS_ERROR_INVALID_OPERATION as _,
    OutOfMemory = ffi::RpsResult_RPS_ERROR_OUT_OF_MEMORY as _,
    FileNotFound = ffi::RpsResult_RPS_ERROR_FILE_NOT_FOUND as _,
    InvalidFileFormat = ffi::RpsResult_RPS_ERROR_INVALID_FILE_FORMAT as _,
    UnsupportedVersionTooOld = ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_OLD as _,
    UnsupportedVersionTooNew = ffi::RpsResult_RPS_ERROR_UNSUPPORTED_VERSION_TOO_NEW as _,
    UnknownNode = ffi::RpsResult_RPS_ERROR_UNKNOWN_NODE as _,
    IndexOutOfBounds = ffi::RpsResult_RPS_ERROR_INDEX_OUT_OF_BOUNDS as _,
    CommandAlreadyFinal = ffi::RpsResult_RPS_ERROR_COMMAND_ALREADY_FINAL as _,
    InteropDataLayoutMismatch = ffi::RpsResult_RPS_ERROR_INTEROP_DATA_LAYOUT_MISMATCH as _,
    KeyNotFound = ffi::RpsResult_RPS_ERROR_KEY_NOT_FOUND as _,
    KeyDuplicated = ffi::RpsResult_RPS_ERROR_KEY_DUPLICATED as _,
    NotImplemented = ffi::RpsResult_RPS_ERROR_NOT_IMPLEMENTED as _,
    IntegerOverflow = ffi::RpsResult_RPS_ERROR_INTEGER_OVERFLOW as _,
    RangeOverlapping = ffi::RpsResult_RPS_ERROR_RANGE_OVERLAPPING as _,
    ValidationFailed = ffi::RpsResult_RPS_ERROR_VALIDATION_FAILED as _,
    InvalidProgram = ffi::RpsResult_RPS_ERROR_INVALID_PROGRAM as _,
    UnsupportedModuleVersion = ffi::RpsResult_RPS_ERROR_UNSUPPORTED_MODULE_VERSION as _,
    TypeMismatch = ffi::RpsResult_RPS_ERROR_TYPE_MISMATCH as _,
    NotSupported = ffi::RpsResult_RPS_ERROR_NOT_SUPPORTED as _,
    RuntimeApiError = ffi::RpsResult_RPS_ERROR_RUNTIME_API_ERROR as _,
    InternalError = ffi::RpsResult_RPS_ERROR_INTERNAL_ERROR as _,
    CodeCount = ffi::RpsResult_RPS_RESULT_CODE_COUNT as _
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
        Result::Ok => Ok(()),
        _ => Err(result)
    }
}
