
use ::prelude::*;

use super::{ slab, buddy };
use super::slab::SlabAllocator;
use super::buddy::BuddyAllocator;
use super::paging::{ PAGE_SIZE, PAGE_SIZE_POWER };

const SLAB_CNT: usize = 5;
const MAX_SIZE_FOR_SLAB: usize = 16 * (1 << (SLAB_CNT - 1));
static mut SLABS: [SlabAllocator<'static>; SLAB_CNT] = unsafe {
    [
        SlabAllocator::new_unchecked(16),
        SlabAllocator::new_unchecked(32),
        SlabAllocator::new_unchecked(64),
        SlabAllocator::new_unchecked(128),
        SlabAllocator::new_unchecked(256),
    ]
};

fn get_slub_num(size: usize) -> usize {
    debug_assert!(size <= 256);
    if size <= 16 {
        0
    } else {
        utility::log2_ceil(size) - 4
    }
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    assert!(8 % _align == 0);
    unsafe {
        *if size <= slab::MAX_FRAME_SIZE {
            SLABS[get_slub_num(size)].allocate()
        } else {
            BuddyAllocator::get_instance().allocate_raw(size).map(|x| x.pointer)
        }.expect("Failed to allocate memory")
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
//    unsafe { libc::free(ptr as *mut libc::c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
//    unsafe {
//        libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
//    }
    0 as *mut _
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

#[cfg(os_test)]
pub mod general_allocator_tests {
    use super::*;
    use super::{ get_slub_num };
    tests_module!("general_allocator",
        get_slub_num_test,
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
}
