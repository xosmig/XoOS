
prelude!();

use ::alloc::arc::Arc;
use ::allocator::buddy::{ BuddyBox, BuddyAllocator };
use ::core::cell::UnsafeCell;

use super::SCHEDULER;


// 16kb for each thread
const THREAD_STACK_BUDDY_LEVEL: usize = 2;

lazy_static! {
    pub static ref MAIN_THREAD_ARC: Arc<ThreadRepr> = Arc::new(ThreadRepr {
        stack: None,                // It already has stack.
        stack_ptr: 0 as *mut _,     // That's fine that the stack pointer for active thread is incorrect.
                                    // The main thread is active at the moment of its initialization
    });
}


/// Inner representation of thread
pub struct ThreadRepr {
    pub stack_ptr: *mut u8,         // will automatically deallocate stack frame
    pub stack: Option<BuddyBox>,    // It is `Option` in order not to allocate anything for main thread.
}
unsafe impl Send for ThreadRepr {}
unsafe impl Sync for ThreadRepr {}


/// A handle to a thread.
pub struct Thread {
    // that's ok to just contain Arc and never use it
    #[allow(dead_code)]  // FIXME
    repr: Arc<ThreadRepr>,
}


/// Used to communicate the return value between the child thread and the parent thread.
struct CompResult<T>(Arc<UnsafeCell<Option<T>>>);
impl<T> Clone for CompResult<T> {
    fn clone(&self) -> Self {
        CompResult(self.0.clone())
    }
}
unsafe impl<T: Send> Send for CompResult<T> {}
unsafe impl<T: Sync> Sync for CompResult<T> {}


/// Provides ownership over thread.
/// Detaches the child thread when `JoinHandle` is dropped.
pub struct JoinHandle<T> {
    // that's ok to just contain Arc and never use it
    #[allow(dead_code)]  // FIXME
    thread: Thread,
    result: CompResult<T>,
}

// Due to reference counting, there is no need to explicitly detach thread in drop.
// There is even no way to do it.
impl<T> JoinHandle<T> {
    // TODO?: panic handling
    pub fn join(self) -> T {
        unsafe {
            // TODO?: sleep (condition variable?)
            // Or I can have a reference to a parent thread to wake it up.
            while (*self.result.0.get()).is_none() {
                SCHEDULER.switch_to_next();  // skip frame
            }
            // It must be the last reference to the child thread
            // because it must be automatically removed after join.
            debug_assert!(Arc::strong_count(&self.result.0) == 1);
            (*self.result.0.get()).take().unwrap()  // return the result of computation
        }
    }
}


/// Spawns a new thread, returning a `JoinHandle` for it.
pub fn spawn<T, F>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static
{
    let comp_res = CompResult(Arc::new(UnsafeCell::new(None)));
    let comp_res_for_closure = comp_res.clone();

    let runnable = move || {
        // TODO: here you theoretically can handle panics
        unsafe {
            *comp_res_for_closure.0.get() = Some(f());
        }
    };

    JoinHandle {
        thread: spawn_runnable(runnable),
        result: comp_res,
    }
}


// FIXME?: it returns `Thread` instead of `FrictionJoint`. M.b. it should be unsafe?
// FIXME: m.b. it should be public (with changed name)
/// Takes function returning unit.
fn spawn_runnable<G>(g: G) -> Thread
    where G: FnOnce() -> () + Send + 'static
{
    let context = Context {
        flags: 0,
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        pbp: 0,
        rbx: Box::into_raw(Box::new(g)) as usize,
        ret_address: real_thread_start::<G> as usize as *const u8,
    };

    // allocate a stack frame for new thread
    let stack = BuddyAllocator::lock().allocate_level(THREAD_STACK_BUDDY_LEVEL)
        .expect("Failed to allocate stack frame for thread.");
    let stack_size = (1 << THREAD_STACK_BUDDY_LEVEL) * mem::paging::PAGE_SIZE as isize;

    // move to the end of stack frame (stack grows down)
    let stack_ptr = unsafe { stack.get().offset(stack_size - size_of::<Context>() as isize) };
    unsafe { ptr::write(stack_ptr as *mut Context, context) };

    let repr = Arc::new(ThreadRepr { stack: Some(stack), stack_ptr: stack_ptr });
    SCHEDULER.add(repr.clone());

    Thread { repr: repr }
}


unsafe extern "C" fn real_thread_start<G>()
    where G: FnOnce() -> () + Send + 'static
{
    let g_ptr: *mut G;
    asm!("movq %rbx, %rax" : "={rax}"(g_ptr) : /*in*/ : /*clb*/ : "volatile");
    let g = Box::from_raw(g_ptr);
    g();

    // FIXME: probably loop is redundant. It must be killed by join and must be never waked up.
    // waiting for join
    loop {
        SCHEDULER.sleep_current();
    }
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
