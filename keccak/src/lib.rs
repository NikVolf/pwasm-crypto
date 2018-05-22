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

macro_rules! keccak_extern {
	($lib_name: ident, $name: ident, $size: expr) => {
		#[no_mangle]
		pub unsafe extern "C" fn $name(src: *const u8, len: u32, dst: *mut u8) {
			let mut keccak = Keccak::$lib_name();
			let res = slice::from_raw_parts_mut(dst, $size/8);
			let source = slice::from_raw_parts(src, len as usize);
			keccak.update(source);
			keccak.finalize(res);
		}
	}
}

keccak_extern!(new_keccak256, keccak256, 256);
keccak_extern!(new_keccak384, keccak384, 384);
keccak_extern!(new_keccak512, keccak512, 512);
