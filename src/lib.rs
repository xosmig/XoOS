
#![feature(associated_consts)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(stmt_expr_attributes)]
#![feature(shared)]
#![feature(nonzero)]
#![feature(naked_functions)]
#![feature(arc_counts)]
#![feature(drop_types_in_const)]
#![feature(allocator)]
#![feature(alloc, collections)]

#![allocator]
#![no_std]

#![cfg_attr(os_test, allow(unused))]

extern crate rlibc;

#[macro_use]
extern crate lazy_static;

/// All code which is necessary to write allocator
#[macro_use] extern crate basics;
extern crate allocator;
extern crate alloc;
#[macro_use] extern crate collections;

pub use ::basics::*;
pub use core::{ cmp, ops, ptr };
pub use ::alloc::boxed;

mod prelude;
pub mod error_handling;
pub mod thread;

use ::boot_info::MultibootInfo;
use ::fmt::Write;

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
    use ::basics::test_lib::run_test_set;

    println!("");
    println!("Run all tests:");
    println!("");

    // run_test_set::<path_to_your_tests_module::Tests>();
    run_test_set::<::test_lib::sample_mod::sample_mod_tests::Tests>();
    run_test_set::<::mem::paging::paging_tests::Tests>();
    run_test_set::<::ioports::ioports_tests::Tests>();
    run_test_set::<::utility::utility_tests::Tests>();
    run_test_set::<::allocator::buddy::buddy_tests::Tests>();
    run_test_set::<::allocator::slab::slab_tests::Tests>();
    run_test_set::<::allocator::allocator_tests::Tests>();
    run_test_set::<::thread::thread_tests::Tests>();

    println!("");
    println!("all tests passed [^_^]");
    println!("");
}


#[cfg(gdb)]
fn gdb_start() {
    let mut gdb_wait = true;
    while unsafe { core::ptr::read_volatile(&gdb_wait) } {  }
}

unsafe fn ini(info_ptr: usize) {
    let info = MultibootInfo::load(info_ptr);
    let mmap = info.memory_map();

    interrupts::init();
    mem::paging::init_default();
    allocator::buddy::BuddyAllocator::init_default(&mmap);
    thread::init();
}


fn end() {
    const OK_MESSAGE: &'static str = "[^_^]";

    println!("{}", OK_MESSAGE);
    vga::print(OK_MESSAGE.as_bytes());
    loop{}
}
