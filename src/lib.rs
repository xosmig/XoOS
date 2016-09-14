#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate libc;

mod error_handling;
mod ioport;
mod vga;
mod serial;
mod utility;

#[allow(unused)]
#[no_mangle]
pub extern fn main() {
    vga::print(b"Hello, World!");

    let mut cout = serial::Serial::get();
    cout.write_string(b"Serial Hello World!");

    loop{}
}
