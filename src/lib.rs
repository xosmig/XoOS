#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]

#![no_std]

extern crate rlibc;

#[macro_use]
pub mod utility;
#[macro_use]
pub mod fmt;
#[macro_use]
pub mod interrupts;

pub mod serial;
pub mod error_handling;
pub mod ioports;
pub mod vga;
pub mod pit;

use fmt::Write;

#[no_mangle]
pub unsafe extern fn rust_start() {
    #[cfg(gdb)] gdb_start();
    ini();

    #[cfg(os_test)] test_all();
    #[cfg(not(os_test))] main();

    end();
}

fn main() {
    pit::unlock_interrupt();
    pit::start_periodical(0xFF_FF);
    unsafe { interrupt!(55) };
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
