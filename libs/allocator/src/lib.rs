
#![feature(associated_consts)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(shared)]
#![feature(nonzero)]
#![feature(step_by)]

#![feature(allocator)]
#![allocator]

#![no_std]

#![cfg_attr(os_test, allow(unused))]

#[macro_use] extern crate basics;

pub use ::basics::*;
use core::{ ptr, cmp };

mod prelude;
pub mod buddy;
pub mod slab;

use slab::SlabAllocator;
use buddy::BuddyAllocator;

const SLAB_CNT: usize = 8;
static mut SLABS: [SlabAllocator<'static>; SLAB_CNT] = unsafe {
    [
        SlabAllocator::new_unchecked(16),
        SlabAllocator::new_unchecked(32),
        SlabAllocator::new_unchecked(64),
        SlabAllocator::new_unchecked(128),
        SlabAllocator::new_unchecked(256),
        SlabAllocator::new_unchecked(512),
        SlabAllocator::new_unchecked(1024),
        SlabAllocator::new_unchecked(slab::MAX_FRAME_SIZE),
    ]
};

fn get_slub_num(size: usize) -> usize {
    debug_assert!(size <= slab::MAX_FRAME_SIZE);
    if size <= 16 {
        0
    } else {
        utility::log2_ceil(size) - 4
    }
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    assert!(8 % align == 0);
    unsafe {
        // there is deref operator to cast from NonZero<*mut u8> to *mut u8
        *if size <= slab::MAX_FRAME_SIZE {
            // use slab allocator for small frames
            SLABS[get_slub_num(size)].allocate()
        } else {
            // use buddy allocator for big frames
            BuddyAllocator::get_instance().allocate_raw(size).map(|x| x.pointer)
        }.expect("Failed to allocate memory")
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, _align: usize) {
    unsafe {
        if old_size <= slab::MAX_FRAME_SIZE {
            // use slab allocator for small frames
            SLABS[get_slub_num(old_size)].deallocate(ptr);
        } else {
            // use buddy allocator for big frames
            BuddyAllocator::get_instance().deallocate_unknown(ptr);
        }
    }

}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    let new_ptr = __rust_allocate(size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(old_size, size)) };
    __rust_deallocate(ptr, old_size, align);
    new_ptr
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size  // FIXME: In fact, it is greater
}

#[cfg(os_test)]
pub mod allocator_tests {
    use super::{ slab, __rust_allocate, __rust_deallocate };
    use super::{ SLABS, get_slub_num };
    tests_module!("allocator",
        get_slub_num_test,
        check_slab_sizes,
        allocate_simple,
    );

    fn get_slub_num_test() {
        assert!(get_slub_num(1) == 0);
        assert!(get_slub_num(2) == 0);
        assert!(get_slub_num(8) == 0);
        assert!(get_slub_num(15) == 0);
        assert!(get_slub_num(16) == 0);
        assert!(get_slub_num(17) == 1);
        assert!(get_slub_num(32) == 1);
        assert!(get_slub_num(33) == 2);
        assert!(get_slub_num(95) == 3);
        assert!(get_slub_num(127) == 3);
        assert!(get_slub_num(128) == 3);
        assert!(get_slub_num(129) == 4);
        assert!(get_slub_num(256) == 4);
    }

    fn check_slab_sizes() {
        unsafe {
            for slab in &SLABS {
                slab.check_correctness();
            }
        }
    }

    fn allocate_simple() {
        const CNT: usize = 16;
        let sizes: [usize; CNT] = [
            1, 2, 4, 8, 16, 24, 32, 72, 160, 256, 1800, slab::MAX_FRAME_SIZE,
            2048, 4000, 4096, 120 * 1024
        ];

        let mut ptrs = [0 as *mut u8; CNT];

        let mut allocate = || {
            for (size, ptr) in sizes.iter().zip(ptrs.iter_mut()) {
                *ptr = __rust_allocate(*size, 8);
            }

            for (size, ptr) in sizes.iter().zip(ptrs.iter()) {
                __rust_deallocate(*ptr, *size, 8);
            }
        };

        allocate();
        allocate();
    }
}
