#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
use core::ptr;


// ABI VERSIONING

pub const ASTRA_ABI_VERSION: u32 = 1;
const RUNTIME_MAJOR: u32 = 0;
const RUNTIME_MINOR: u32 = 1;
const RUNTIME_PATCH: u32 = 0;

#[repr(C)]
pub struct astra_runtime_info {
    pub abi_version: u32,
    pub runtime_major: u32,
    pub runtime_minor: u32,
    pub runtime_patch: u32,
}


// STATUS CODES

#[repr(C)]
#[derive(Copy, Clone)]
pub enum astra_status {
    ASTRA_OK = 0,
    ASTRA_ERR_INCOMPATIBLE_ABI = 1,
    ASTRA_ERR_INTERNAL = 255,
}

// ABI ENTRY POINT



/// Writes runtime information to `out`.
///
/// # Safety
///
/// The caller must ensure that `out`:
/// - is non-null
/// - points to valid, writable memory
/// - is properly aligned for `astra_runtime_info`
///
/// Violating this contract results in undefined behavior.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn astra_runtime_get_info(
    out: *mut astra_runtime_info,
) -> astra_status {
    // Check if the output pointer is null
    if out.is_null() {
        return astra_status::ASTRA_ERR_INTERNAL;
    }

   
    // Create the runtime info structure
    let runtime_info = astra_runtime_info {
        abi_version: ASTRA_ABI_VERSION,
        runtime_major: RUNTIME_MAJOR,
        runtime_minor: RUNTIME_MINOR,
        runtime_patch: RUNTIME_PATCH,
    };
       
    unsafe {
        ptr::write(out, runtime_info);
    }

    astra_status::ASTRA_OK
}
