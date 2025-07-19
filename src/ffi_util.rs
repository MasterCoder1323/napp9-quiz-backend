use libc::c_char;
use serde::de::DeserializeOwned;
use std::ffi::{CStr};
use std::ptr;

/// Read a JSON value from the 4KB C buffer.
pub fn read_json_from_buffer<T: DeserializeOwned>(buffer: *mut c_char) -> Result<T, &'static str> {
    if buffer.is_null() {
        return Err("input null");
    }

    let cstr = unsafe { CStr::from_ptr(buffer) };
    let json_str = cstr.to_str().map_err(|_| "invalid utf8")?;

    serde_json::from_str(json_str).map_err(|_| "invalid json")
}

/// Write a UTF-8 string into the 4KB C buffer, safely truncated.
pub fn write_str_to_buffer(buffer: *mut c_char, result: &str) {
    if buffer.is_null() {
        return;
    }

    const MAX_SIZE: usize = 4095; // reserve 1 byte for null
    let bytes = result.as_bytes();
    let copy_len = bytes.len().min(MAX_SIZE);

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buffer as *mut u8, copy_len);
        *buffer.add(copy_len) = 0; // Null terminator
    }
}
