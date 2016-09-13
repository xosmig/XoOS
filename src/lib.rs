#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate libc;

use libc::{c_void, c_int, size_t};

mod error_handling;

static GDB_FLAG: bool = true;

#[no_mangle]
pub extern fn main() {
    while unsafe { core::ptr::read_volatile(&GDB_FLAG) } {}

    let x = ["Hello", "World", "!"];
    let y = x;

    let test = (0..3).flat_map(|x| 0..x).zip(0..);

    loop {}
}
