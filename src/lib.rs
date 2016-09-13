#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate libc;

use libc::{c_void, c_int, size_t};

mod error_handling;
mod ioport;
mod vga;

#[no_mangle]
pub extern fn main() {
    vga::print(b"Hello World! I'm cat.");

    loop{}
}
