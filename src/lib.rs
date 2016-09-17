#![feature(lang_items)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]

#![no_std]
#![allow(unused)] // FIXME

extern crate rlibc;
extern crate libc;

#[macro_use] mod fmt;
#[macro_use] mod serial;
#[macro_use] mod interrupts;
mod error_handling;
mod ioport;
mod vga;
mod utility;

use fmt::Write;

#[no_mangle]
pub extern fn main() {
    #[cfg(gdb)] gdb_start();
    #[cfg(os_test)] test_all();

    end();
}

#[cfg(os_test)]
fn test_all() {
    fmt::tests::all();
}

#[cfg(gdb)]
fn gdb_start() {
    {
        let mut gdb_wait = true;
        while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
    }
}

fn end() {
    const OK_MESSAGE: &'static [u8] = b"[^_^]";

//    Serial::get().write_str(OK_MESSAGE);
//    Serial::get().write_str(b"\n");
    vga::print(OK_MESSAGE);
    loop{}
}
