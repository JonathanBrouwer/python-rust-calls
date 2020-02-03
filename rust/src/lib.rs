#![crate_type = "dylib"]

#[no_mangle]
pub extern "C" fn inc(x: i32) -> i32 {
    x + 1
}