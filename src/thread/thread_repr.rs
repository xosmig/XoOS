
use ::alloc::arc::Arc;
use ::collections::string::String;
use ::sync::{ SpinLock, LockGuard };
use ::core::marker::PhantomData;

use ::allocator::buddy::{ BuddyBox, BuddyAllocator };

pub struct ThreadRepr {
    pub name: Option<String>,
    pub stack_ptr: *mut (),
    pub stack: BuddyBox,
}

unsafe impl Send for ThreadRepr {}
unsafe impl Sync for ThreadRepr {}
