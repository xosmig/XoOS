
#![feature(associated_consts)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(shared)]
#![feature(nonzero)]

#![no_std]

#![cfg_attr(os_test, allow(unused))]

#[cfg(os_test)] #[macro_use] pub mod test_lib_macro;
#[macro_use] pub mod utility_macro;
#[macro_use] pub mod fmt;

mod prelude;
#[cfg(os_test)] pub mod test_lib;
pub mod utility;
pub mod ioports;
pub mod serial;
pub mod mem;
pub mod boot_info;

