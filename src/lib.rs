#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]
#![feature(shared)]

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
pub mod mem;

use fmt::Write;
use boot_info::MultibootInfo;
use mem::memory_map::MemoryMap;

#[no_mangle]
pub unsafe extern fn rust_start(info_ptr: usize) {
    #[cfg(gdb)] gdb_start();
    ini(info_ptr);

    #[cfg(os_test)] test_all();
    #[cfg(not(os_test))] main();

    end();
}


fn main() {

}


#[cfg(os_test)]
fn test_all() {
    fmt::tests::all();
    ioports::ioports_tests::all();
    utility::utility_tests::all();
    mem::paging::tests::all();
}


#[cfg(gdb)]
fn gdb_start() {
    let mut gdb_wait = true;
    while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
}


unsafe fn ini(info_ptr: usize) {
    let info = MultibootInfo::load(info_ptr);
    let mmap = info.memory_map();

    interrupts::init_default();
    mem::paging::init_default();
    mem::buddy::init_default(&mmap);
}


fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}
