
mod inplace_list;

pub mod paging;
pub mod memory_map;

pub mod buddy;
pub mod slab;
pub mod general_allocator;

pub use ::core::mem::*;

use self::paging::PAGE_SIZE;
use self::paging::MEMORY_START;

fn get_mut_ptr<T>(phys_address: usize) -> *mut T {
    (phys_address + MEMORY_START) as *mut T
}
