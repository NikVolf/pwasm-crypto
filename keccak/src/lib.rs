#![no_std]
extern crate pwasm_std;
extern crate tiny_keccak;

use tiny_keccak::Keccak;
use core::slice;

#[no_mangle]
pub fn call() {
}

#[no_mangle]
pub fn deploy() {
}

#[no_mangle]
pub unsafe extern "C" fn keccak256(src: *const u8, len: u32, dst: *mut u8) {
	let mut keccak = Keccak::new_keccak256();
	let res = slice::from_raw_parts_mut(dst, 32);
	let source = slice::from_raw_parts(src, len as usize);
	keccak.update(source);
	keccak.finalize(res);
}

#[no_mangle]
pub unsafe extern "C" fn keccak384(src: *const u8, len: u32, dst: *mut u8) {
	let mut keccak = Keccak::new_keccak384();
	let res = slice::from_raw_parts_mut(dst, 48);
	let source = slice::from_raw_parts(src, len as usize);
	keccak.update(source);
	keccak.finalize(res);
}

#[no_mangle]
pub unsafe extern "C" fn keccak512(src: *const u8, len: u32, dst: *mut u8) {
	let mut keccak = Keccak::new_keccak512();
	let res = slice::from_raw_parts_mut(dst, 64);
	let source = slice::from_raw_parts(src, len as usize);
	keccak.update(source);
	keccak.finalize(res);
}