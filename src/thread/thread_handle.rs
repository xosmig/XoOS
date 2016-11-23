
prelude!();

use ::core::marker::PhantomData;
use ::alloc::arc::Arc;
use ::allocator::buddy::{ BuddyBox, BuddyAllocator };
use ::core::cell::UnsafeCell;

use super::SCHEDULER;


// 16kb
const STACK_BUDDY_LEVEL: usize = 2;


pub struct ThreadRepr {
    pub stack_ptr: *mut u8,
    pub stack: BuddyBox,
}
unsafe impl Send for ThreadRepr {}
unsafe impl Sync for ThreadRepr {}


pub struct Thread {
    repr: Arc<ThreadRepr>,
}

//#[derive(Clone)]  // I wonder why it doesn't work
struct CompResult<T>(Arc<UnsafeCell<Option<T>>>);
impl<T> Clone for CompResult<T> {
    fn clone(&self) -> Self {
        CompResult(self.0.clone())
    }
}
unsafe impl<T: Send> Send for CompResult<T> {}
unsafe impl<T: Sync> Sync for CompResult<T> {}


/// Provides ownership over thread.
/// Detaches underlying thread when it is dropped.
pub struct JoinHandle<T> {
    thread: Thread,
    result: CompResult<T>,
}

impl<T> JoinHandle<T> {
    // TODO?: result of computation
    // TODO?: panic handling
    pub fn join(self) -> Option<T> {
        // TODO: join child thread
        panic!("");
        unsafe { (*self.result.0.get()).take() }
    }
}

//pub fn sleep() {
// TODO
//}


//pub fn current<T>() -> Thread<T> {
//}


// FIXME: f probably should be FnOnce
pub fn spawn<T, F>(mut f: F) -> JoinHandle<T>
    where F: FnMut() -> T + Send + 'static, T: Send + 'static
{
    let comp_res = CompResult(Arc::new(UnsafeCell::new(None)));
    let comp_res_in_f = comp_res.clone();

    let f_wrapped = move || {
        // TODO: here you theoretically can handle panics
        unsafe {
            *comp_res_in_f.0.get() = Some(f());
        }
    };

    JoinHandle {
        thread: spawn_impl(f_wrapped),
        result: comp_res,
    }
}


// FIXME: it returns `Thread`. M.b. it should be unsafe?
// FIXME: m.b. it should be public (with changed name)
/// Takes function without return value.
fn spawn_impl<G>(mut runnable: G) -> Thread
    where G: FnMut() -> () + Send + 'static
{
    unsafe extern "C" fn real_thread_start<G>()
        where G: FnMut() -> () + Send + 'static
    {
        let runnable: *mut G;
        asm!("movq %rbx, %ax" : "={ax}"(runnable) : /*in*/ : /*clb*/ : "volatile");
        (*runnable)();

        loop {}  // FIXME: sleep until join
        // TODO: ???
    }

    let mut context = Context {
        flags: 0,
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        pbp: 0,
        rbx: &mut runnable as *mut G as usize,
        ret_address: real_thread_start::<G> as usize as *const u8,
    };

    let stack = BuddyAllocator::get_instance().allocate_level(STACK_BUDDY_LEVEL)
        .expect("Failed to allocate stack frame for thread.");
    let stack_size = (1 << STACK_BUDDY_LEVEL) * mem::paging::PAGE_SIZE as isize;

    // move to the end of page (stack grows down)
    let stack_ptr = unsafe { stack.get().offset(stack_size - size_of::<Context>() as isize) };
    unsafe { ptr::write(stack_ptr as *mut Context, context) };

    let repr = Arc::new(ThreadRepr { stack: stack, stack_ptr: stack_ptr });
    SCHEDULER.lock().add(repr.clone());

    Thread { repr: repr }
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
