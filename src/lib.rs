#![feature(lang_items)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![no_std]

extern crate rlibc;
extern crate libc;

mod error_handling;
mod ioport;
mod vga;
#[macro_use] mod serial;
mod utility;
mod fmod;

pub use fmod::*;

use serial::Serial;
//#[macro_use]
use serial::fmt::*;

const OK_MESSAGE: &'static [u8] = b"[^_^]";

#[cfg(gdb)]
static GDB_WAIT: bool = true;

#[no_mangle]
pub extern fn main() {
    #[cfg(gdb)]
    {
        while unsafe { core::ptr::read_volatile(&GDB_WAIT) } {  }
    }

    let x = 12;
    let msg1 = "Hello, World. This is "; // FIXME
    let msg2 = " in octal: ";
    let msg3 = ".\n";
    print!(msg1, x, msg2, octal(&x), msg3);

    Serial::get().write_str(OK_MESSAGE);
    Serial::get().write_str(b"\n");
    vga::print(OK_MESSAGE);
    loop{}
}
