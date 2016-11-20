#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]
#![feature(shared)]
#![feature(nonzero)]
#![feature(step_by)]
#![feature(associated_consts)]

#![no_std]

#![allow(unused_unsafe)]
#![allow(unused_imports)]

extern crate rlibc;

#[macro_use] pub mod macro_utility;
#[macro_use] pub mod fmt;
#[cfg(os_test)] #[macro_use] pub mod tests_macro;
#[cfg(os_test)] pub mod tests;
#[macro_use] pub mod interrupts;

pub mod prelude;
pub mod utility;
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

    #[cfg(os_test)] tests::test_all();
    #[cfg(not(os_test))] main();

    end();
}

#[cfg(not(os_test))]
fn main() {
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
    mem::buddy::BuddyAllocator::init_default(&mmap);
}


fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}
