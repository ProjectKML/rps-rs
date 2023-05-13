macro_rules! define_handle {
    ($name: ident) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(*mut u8);

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::null()
            }
        }

        impl $name {
            #[inline]
            pub fn from_raw(raw: *mut u8) -> Self {
                Self(raw)
            }

            #[inline]
            pub fn into_raw(self) -> *mut u8 {
                self.0
            }

            #[inline]
            pub const fn null() -> Self {
                Self(::std::ptr::null_mut())
            }
        }

        impl ::std::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Pointer::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

pub(crate) use define_handle;

macro_rules! assert_size_and_align {
    ($type_: ty, $ffi_type: ty) => {
        ::static_assertions::assert_eq_size!($type_, $ffi_type);
        ::static_assertions::assert_eq_align!($type_, $ffi_type);
    };
}

pub(crate) use assert_size_and_align;
