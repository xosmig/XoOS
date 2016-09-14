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
mod fmod;

pub use fmod::*;

use core::fmt;
use serial::Serial;

const OK_MESSAGE: &'static [u8] = b"[^_^]";

#[allow(unused)]
#[no_mangle]
pub extern fn main() {
    struct Test;
    impl fmt::Display for Test {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            panic!();
        }
    }
    format_args!("{}", Test {});

    Serial::get().write_str(OK_MESSAGE);
    vga::print(OK_MESSAGE);

    loop{}
}
