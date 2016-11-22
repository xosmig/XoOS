
// TODO: remove some guards

prelude!();

mod int_mutex;
mod pit;
mod thread_handle;

pub use self::thread_handle::*;

use self::int_mutex::*;

use ::alloc::arc::Arc;
use ::collections::string::String;
use ::sync::{ SpinLock, LockGuard };
use ::core::marker::PhantomData;
use ::collections::VecDeque;


const TIME_UNIT: u16 = 256;


lazy_static! {
    static ref SCHEDULER: IntMutex<Scheduler> = IntMutex::new(Scheduler::new());
}


struct Scheduler {
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
        self.queue.push_back(thread);
    }

    // FIXME: assumes that there is at least one active thread.
    /// Removes current thread from queue.
    pub fn sleep_current(&mut self) {
                let prev_arc = self.queue.pop_front().unwrap();
                let next = self.queue.front().unwrap().as_ref() as *const _;

                unsafe { self.switch_threads(prev_arc.as_ref() as *const _, next) };

                self.refresh_timer();
    }

    // FIXME: assumes that there is at least one active thread.
    pub fn switch_to_next(&mut self) {
                let th = self.queue.pop_front().unwrap();
                let prev = th.as_ref() as *const _;
                let next = self.queue.front().unwrap().as_ref() as *const _;
                self.queue.push_back(th);

                unsafe { self.switch_threads(prev, next) };
    }

    /// It's inside `Scheduler` to ensure thread safety
    fn refresh_timer(&mut self) {
        unsafe { pit::start_periodical(TIME_UNIT) };
    }

    /// It's inside `Scheduler` to ensure thread safety
    unsafe fn switch_threads(&mut self, prev: *const ThreadRepr, next: *const ThreadRepr) {
        let prev = prev as *mut ThreadRepr;
        switch_threads_impl(&mut (*prev).stack_ptr, (*next).stack_ptr);

        #[naked]
        unsafe fn switch_threads_impl(_prev: &mut *mut u8, _next: *mut u8) {
            asm!("
                pushq %rbx
                pushq %rbp
                pushq %r12
                pushq %r13
                pushq %r14
                pushq %r15
                pushfq

                movq %rsp, (%rdi)
                movq %rsi, %rsp

                popfq
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


/// Should be called only by corresponding interruption handler.
#[no_mangle]
pub unsafe fn __kernel_timer_tick() {
    // EOI must be sent before switching context
    ::interrupts::pic::PIC_1.end_of_interrupt();
    SCHEDULER.lock().switch_to_next();
}
