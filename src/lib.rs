
#![feature(associated_consts)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]
#![feature(shared)]
#![feature(nonzero)]
#![feature(allocator)]

#![allocator]

#![no_std]

#![cfg_attr(os_test, allow(unused))]

extern crate rlibc;

/// All code which is necessary to write allocator
#[macro_use]
extern crate basics;

//extern crate allocator;

#[macro_use] pub mod interrupts;

mod prelude;
pub mod error_handling;
pub mod vga;
pub mod pit;
//pub mod mem;

//use ::prelude::light::*;
//use boot_info::MultibootInfo;
//use mem::memory_map::MemoryMap;


#[no_mangle]
pub unsafe extern fn rust_start(info_ptr: usize) {
//    #[cfg(gdb)] gdb_start();
//    ini(info_ptr);
//
//    #[cfg(os_test)] tests::test_all();
//    #[cfg(not(os_test))] main();
//
//    end();
}

fn main() {
}


/*
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
*/
