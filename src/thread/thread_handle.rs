
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

// Due to reference counter, there is no need to explicitly detach thread in drop.
// There is even no way to do it.
impl<T> JoinHandle<T> {
    // TODO?: panic handling
    pub fn join(self) -> T {
        unsafe {
            // TODO: sleep (condition variable)
            while (*self.result.0.get()).is_none() {}
            // The child thread must be removed after join.
            debug_assert!(Arc::strong_count(&self.result.0) == 1);
            (*self.result.0.get()).take().unwrap()
        }
    }
}

//pub fn sleep() {
// TODO
//}


//pub fn current<T>() -> Thread<T> {
//}


/// original documentation: https://doc.rust-lang.org/nightly/std/
///
/// Spawns a new thread, returning a `JoinHandle` for it.
///
/// The join handle will implicitly *detach* the child thread upon being
/// dropped. In this case, the child thread may outlive the parent (unless
/// the parent thread is the main thread; the whole process is terminated when
/// the main thread finishes.) Additionally, the join handle provides a `join`
/// method that can be used to join the child thread. If the child thread
/// panics, `join` will return an `Err` containing the argument given to
/// `panic`.
pub fn spawn<T, F>(mut f: F) -> JoinHandle<T>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static
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
fn spawn_impl<G>(mut g: G) -> Thread
    where G: FnOnce() -> () + Send + 'static
{
    unsafe extern "C" fn real_thread_start<G>()
        where G: FnOnce() -> () + Send + 'static
    {
        let g_ptr: *mut G;
        asm!("movq %rbx, %ax" : "={ax}"(g_ptr) : /*in*/ : /*clb*/ : "volatile");
        let g = Box::from_raw(g_ptr);
        g();

        //FIXME: probably loop is redundant.It must be killed by join and must not be waked up ever.
        // waiting for join
        loop {
            SCHEDULER.lock().sleep_current();
        }
        unreachable!();
    }

    let mut context = Context {
        flags: 0,
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        pbp: 0,
        rbx: Box::into_raw(Box::new(g)) as usize,
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
