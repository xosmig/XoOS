#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate libc;

use libc::{c_void, c_int, size_t};

static GDB_FLAG: bool = true;

#[no_mangle]
pub extern fn main() {
    while unsafe { core::ptr::read_volatile(&GDB_FLAG) } {}

    let x = ["Hello", "World", "!"];
    let y = x;

    loop {}
}

// Used for Rust’s unwinding on panic.
#[lang = "eh_personality"] extern fn eh_personality() {}

// The entry point on panic.
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {} // FIXME
}
