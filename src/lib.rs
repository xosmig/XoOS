#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]

#![no_std]

extern crate rlibc;

#[macro_use]
pub mod fmt;
#[macro_use]
pub mod interrupts;

pub mod serial;
pub mod error_handling;
pub mod ioports;
pub mod vga;
pub mod utility;
pub mod pit;

use fmt::Write;

#[no_mangle]
pub unsafe extern fn main() {
    #[cfg(gdb)] gdb_start();
    #[cfg(os_test)] test_all();
    println!("Hello, World");
    ini();

    interrupt!(55);

    end();
}

#[cfg(os_test)]
fn test_all() {
    fmt::tests::all();
    ioports::tests::all();
}

#[cfg(gdb)]
fn gdb_start() {
    let mut gdb_wait = true;
    while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
}

unsafe fn ini() {
    interrupts::init_default();
}

fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}
