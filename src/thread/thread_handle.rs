
prelude!();

use ::core::marker::PhantomData;
use ::alloc::arc::Arc;
use ::allocator::buddy::{ BuddyBox, BuddyAllocator };

use super::SCHEDULER;


// 16kb
const STACK_BUDDY_LEVEL: usize = 2;


pub struct ThreadRepr {
    pub stack_ptr: *mut u8,
    pub stack: BuddyBox,
}

unsafe impl Send for ThreadRepr {}
unsafe impl Sync for ThreadRepr {}


#[derive(Default)]
pub struct Thread<T> {
    repr: Option<Arc<ThreadRepr>>,
    phantom: PhantomData<T>,
}

impl<T> Thread<T> {
    // TODO?: result of computation
    // TODO?: panic handling
    pub fn join() /*-> Result<T>*/ {
        // TODO
//        unsafe { mem::uninitialized() }
    }

    pub fn detach(&mut self) {
        self.repr = None;
    }
}

//pub fn sleep() {
// TODO
//}


// FIXME: f probably should be FnOnce
pub fn spawn<T, F>(mut f: F) -> Thread<T>
    where F: FnMut() -> T, F: Send + 'static, T: Send + 'static
{
    unsafe extern "C" fn thread_start<T, F>() -> T
        where F: FnMut() -> T, F: Send + 'static, T: Send + 'static
    {
        let f: *mut F;
        asm!("movq %r15, %ax" : "={ax}"(f) : /*in*/ : /*clb*/ : "volatile");
        let ret = (*f)();

        loop {}  // FIXME: sleep until join
        ret
    }

    let mut context = Context {
        flags: 0,
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        pbp: 0,
        rbx: &mut f as *mut F as usize,
        ret_address: thread_start::<T, F> as usize as *const u8,
    };

    let stack = BuddyAllocator::get_instance().allocate_level(STACK_BUDDY_LEVEL)
        .expect("Failed to allocate stack frame for thread.");
    let stack_size = (1 << STACK_BUDDY_LEVEL) * mem::paging::PAGE_SIZE as isize;
    // move to the end of page (stack grows down)

    let stack_ptr = unsafe { stack.get().offset(stack_size - size_of::<Context>() as isize) };
    unsafe { ptr::write(stack_ptr as *mut Context, context) };

    let repr = Arc::new(ThreadRepr { stack: stack, stack_ptr: stack_ptr });
    SCHEDULER.lock().add(repr.clone());

    Thread { repr: Some(repr), phantom: PhantomData }
}


#[repr(C)]
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct Context {
    flags: usize,
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    pbp: usize,
    rbx: usize,
    ret_address: *const u8,
}
