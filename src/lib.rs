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
#[macro_use] mod interrupts;

use serial::Serial;

#[no_mangle]
pub extern fn main() {
    gdb_start();

    interrupts::lock_on_cpu();
//    unsafe { interrupt!(105) };

    end();
}

#[cfg(gdb)]
static GDB_WAIT: bool = true;

fn gdb_start() {
    #[cfg(gdb)]
    {
        while unsafe { core::ptr::read_volatile(&GDB_WAIT) } {  }
    }
}

fn end() {
    const OK_MESSAGE: &'static [u8] = b"[^_^]";

    Serial::get().write_str(OK_MESSAGE);
    Serial::get().write_str(b"\n");
    vga::print(OK_MESSAGE);
    loop{}
}
