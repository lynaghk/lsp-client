#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::all)]
#![allow(improper_ctypes)]

pub mod bindings;
pub mod foo;

pub mod c_types {
    pub type c_void = std::os::raw::c_void;
    pub type c_uchar = std::os::raw::c_uchar;
    pub type c_schar = std::os::raw::c_schar;
    pub type c_char = std::os::raw::c_char;
    pub type c_short = std::os::raw::c_short;
    pub type c_ushort = std::os::raw::c_ushort;
    pub type c_int = std::os::raw::c_int;
    pub type c_uint = std::os::raw::c_uint;
    pub type c_long = std::os::raw::c_long;
    pub type c_ulong = std::os::raw::c_ulong;
    pub type c_longlong = std::os::raw::c_longlong;
    pub type c_ulonglong = std::os::raw::c_ulonglong;
}
