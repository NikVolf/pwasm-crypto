#![no_std]

extern crate pwasm_std;

#[no_mangle]
pub fn call() {
}

#[no_mangle]
pub fn deploy() {
}

#[no_mangle]
pub fn hash(src: *const u8, len: u32, dst: *mut u8) {
}