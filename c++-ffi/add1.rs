#![crate_type = "staticlib"]

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
/// Add 1 to the number referenced by `x`, print the result of the add along with `msg`, return
/// the result.
///
/// # Safety
///
/// `msg` must point to a valid C string, and `x` must point to a valid C `uint64_t`.
pub unsafe extern "system" fn add1(x: &mut i64, msg: *const c_char) -> i64 {
    let msg = CStr::from_ptr(msg);
    *x += 1;
    println!("{} {}", msg.to_string_lossy(), *x);
    *x
}
