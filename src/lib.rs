#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]
#![feature(try_from)]

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
pub mod boot_info;
pub mod memory_map;
pub mod paging;

use fmt::Write;
use boot_info::*;

#[no_mangle]
pub unsafe extern fn rust_start(info_ptr: usize) {
    #[cfg(gdb)] gdb_start();
    ini();

    #[cfg(os_test)] test_all();
    #[cfg(not(os_test))] main(MultibootInfo::load(info_ptr));

    end();
}


fn main(info: &'static MultibootInfo) {
    let mem_map = info.memory_map();
    println!("{:?}", mem_map);
}


#[cfg(os_test)]
fn test_all() {
    fmt::tests::all();
    ioports::tests::all();
    paging::tests::all();
}


#[cfg(gdb)]
fn gdb_start() {
    let mut gdb_wait = true;
    while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
}


unsafe fn ini() {
    interrupts::init_default();
    paging::init_default();
}


fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}
