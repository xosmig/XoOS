
pub mod paging;
pub mod memory_map;

mod buddy;

pub use ::core::mem::*;

use self::paging::PAGE_SIZE;
