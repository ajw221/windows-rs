// Bindings generated by `windows-bindgen` 0.48.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[link(name = "kernel32")]
extern "system" {
    pub fn CloseHandle(hobject: HANDLE) -> BOOL;
}
#[link(name = "kernel32")]
extern "system" {
    pub fn GetLastError() -> WIN32_ERROR;
}
#[link(name = "ws2_32")]
extern "system" {
    pub fn socket(af: i32, r#type: WINSOCK_SOCKET_TYPE, protocol: i32) -> SOCKET;
}
pub type BCRYPT_ALG_HANDLE = *mut ::core::ffi::c_void;
pub type BOOL = i32;
pub type FindFileHandle = *mut ::core::ffi::c_void;
pub type HANDLE = *mut ::core::ffi::c_void;
pub type HMODULE = *mut ::core::ffi::c_void;
#[cfg(target_pointer_width = "32")]
pub type SOCKET = u32;
#[cfg(target_pointer_width = "64")]
pub type SOCKET = u64;
pub type WIN32_ERROR = u32;
pub type WINSOCK_SOCKET_TYPE = i32;