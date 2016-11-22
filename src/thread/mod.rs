
// TODO: remove some guards

prelude!();

mod int_mutex;
mod pit;
mod scheduler;
mod thread_repr;

//use self::int_guard::*;
use self::thread_repr::*;
use self::scheduler::*;
use self::int_mutex::*;

use ::alloc::arc::Arc;
use ::collections::string::String;
use ::sync::{ SpinLock, LockGuard };
use ::core::marker::PhantomData;
use ::collections::VecDeque;


const TIME_UNIT: u16 = 256;


/*#[repr(C)]
#[repr(packed)]
#[derive(Debug)]
struct Context {
    flags: usize,
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    pbp: usize,
    rbx: usize,
    ret_address: *const (),
}*/


pub struct Scheduler {
    // TODO: inplace queue
    queue: VecDeque<Arc<ThreadRepr>>,
}


impl Scheduler {
    pub fn new() -> Self {
        let mut res = Scheduler { queue: VecDeque::new() };
        res.refresh_timer();
        res
    }

    /// Adds new thread to the queue.
    pub fn add(&mut self, thread: Arc<ThreadRepr>) {
        //        let guard = IntGuard::new();
        self.queue.push_back(thread);
    }

    // FIXME: assumes that there is at least one active thread.
    /// Removes current thread from queue.
    pub fn sleep_current(&mut self) {
        /*        let guard = IntGuard::new();

                let prev_arc = self.queue.pop_front().unwrap();
                let next = self.queue.front().unwrap().as_ref() as *const _;

                unsafe { switch_threads(prev_arc.as_ref() as *const _, next) };

                refresh_timer();*/
    }

    // FIXME: assumes that there is at least one active thread.
    pub fn switch_to_next(&mut self) {
        /*        let guard = IntGuard::new();

                let th = self.queue.pop_front().unwrap();
                let prev = th.as_ref() as *const _;
                let next = self.queue.front().unwrap().as_ref() as *const _;
                self.queue.push_back(th);

                unsafe { switch_threads(prev, next) };*/
    }

    fn refresh_timer(&mut self) {
        unsafe { pit::start_periodical(TIME_UNIT) };
    }

    unsafe fn switch_threads(&mut self, prev: *const ThreadRepr, next: *const ThreadRepr) {
        let prev = prev as *mut ThreadRepr;
        switch_threads_impl(&mut (*prev).stack_ptr, (*next).stack_ptr);

        #[naked]
        unsafe fn switch_threads_impl(_prev: &mut *mut (), _next: *mut ()) {
            asm!("
                pushq %rbx
                pushq %rbp
                pushq %r12
                pushq %r13
                pushq %r14
                pushq %r15

                movq %rsp, (%rdi)
                movq %rsi, %rsp

                popq %r15
                popq %r14
                popq %r13
                popq %r12
                popq %rbp
                popq %rbx

                ret
            "
            : /*out*/
            : /*in*/
            : /*clb*/
            : "volatile");
        }
    }
}



#[derive(Default)]
pub struct Thread<T> {
    repr: Option<Arc<ThreadRepr>>,
    phantom: PhantomData<T>,
}

impl<T> Thread<T> {
    // TODO?: panic handling
    pub fn join() -> T /*Result<T>*/ {
        // TODO
        unsafe { mem::uninitialized() }
    }

    pub fn detach(&mut self) {
        self.repr = None;
    }
}

impl<T> Drop for Thread<T> {
    fn drop(&mut self) {
        // TODO: detach
    }
}


pub fn sleep() {
    // TODO
}

pub fn spawn<T, F>(f: F) -> Thread<T>
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    // TODO
    unsafe { mem::uninitialized() }
}


lazy_static! {
    static ref SCHEDULER: IntMutex<Scheduler> = IntMutex::new(Scheduler::new());
}


/// Should be called only by corresponding interruption handler.
#[no_mangle]
pub unsafe fn __kernel_timer_tick() {
    // TODO
}
