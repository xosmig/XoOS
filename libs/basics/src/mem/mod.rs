
pub mod inplace_list;

pub mod paging;
pub mod memory_map;

pub use ::core::mem::*;

use self::paging::MEMORY_START;

pub fn get_mut_ptr<T>(phys_address: usize) -> *mut T {
    (phys_address + MEMORY_START) as *mut T
}
