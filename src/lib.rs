#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]

#![no_std]
#![allow(unused)] // FIXME

extern crate rlibc;
extern crate libc;

#[macro_use] mod fmt;
#[macro_use] mod interrupts;
mod serial;
mod error_handling;
mod ioport;
mod vga;
mod utility;

use fmt::Write;

#[no_mangle]
pub unsafe extern fn main() {
    #[cfg(gdb)] gdb_start();
    #[cfg(os_test)] test_all();
    ini();

    asm!("INT 63" : /*out*/ : /*in*/ : /*clb*/ : "volatile", "intel");

    end();
}

#[cfg(os_test)]
fn test_all() {
    fmt::tests::all();
}

#[cfg(gdb)]
fn gdb_start() {
    let mut gdb_wait = true;
    while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
}

unsafe fn ini() {
    interrupts::idt::setup();
    interrupts::unlock_on_cpu();
}

fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    // just sleep for some time
    for i in 0..1_000_000 {
        // do_nothing
    }

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}

// for visibility from asm.
pub use ::interrupts::idt::handle_interrupt;

