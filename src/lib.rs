#![feature(lang_items)]
#![feature(asm)]
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

//    println!("{}", core::mem::size_of::<interrupts::idt::IdtItem>());
//    asm!("INT 0xd" : /*out*/ : /*in*/ : /*clb*/ : "volatile", "intel");
//    interrupts::unlock_on_cpu();

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
    interrupts::idt::mysetup();
}

fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}

// for visibility from asm.
pub use ::interrupts::idt::handle_interrupt;

