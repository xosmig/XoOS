
prelude!();

//mod int_mutex;
mod pit;
mod thread_impl;

pub use self::thread_impl::{ spawn, Thread, JoinHandle };
//use self::int_mutex::*;

use self::thread_impl::ThreadRepr;
use ::alloc::arc::Arc;
use ::collections::VecDeque;
use ::core::cell::UnsafeCell;


// FIXME
const TIME_FRAME: u16 = 256 * 255;

extern "C" {
    fn switch_thread_context(_prev /*rdi*/: &mut *mut u8, _next /*rsi*/: *mut u8);
}


static SCHEDULER: Scheduler = unsafe { Scheduler::uninitialized() };

type QueueT = VecDeque<Arc<ThreadRepr>>;

struct Scheduler {
    queue: UnsafeCell<Option<QueueT>>,
    main_thread: UnsafeCell<Option<Arc<ThreadRepr>>>,
}
unsafe impl Sync for Scheduler {}
unsafe impl Send for Scheduler {}

impl Scheduler {
    const unsafe fn uninitialized() -> Self {
        Scheduler { queue: UnsafeCell::new(None), main_thread: UnsafeCell::new(None) }
    }

    unsafe fn init(&self) {
        // initialize queue
        (*self.queue.get()) = Some(VecDeque::new());

        // initialize main thread object
        (*self.main_thread.get()) = Some(Arc::new(
            ThreadRepr {
                stack: None,                // It already has stack.
                stack_ptr: 0 as *mut _,     // That's fine that the stack pointer for active thread is incorrect.
                                            // The main thread is active at the moment of its initialization
            }
        ));

        // we need to add main thread in the queue
        self.get().push_back((&*self.main_thread.get()).as_ref().unwrap().clone());

        self.refresh_timer();
        pit::unlock_interrupt();
    }

    /// provides easy inner mutability
    fn get(&self) -> &mut QueueT {
        unsafe { (*self.queue.get()).as_mut().unwrap() }
    }

    /// Adds new thread to the queue.
    pub fn add(&self, thread: Arc<ThreadRepr>) {
        self.get().push_back(thread);
    }

    // FIXME?: assumes that there is at least one active thread.
    /// Removes current thread from queue.
    pub fn sleep_current(&self) {
        unsafe { ::interrupts::lock_on_cpu() };
        debug_assert!(self.get().len() >= 1);

        let prev_arc = self.get().pop_front().unwrap();
        let prev = prev_arc.as_ref() as *const _ as *mut _;
        let next = self.get().front().unwrap().as_ref() as *const _ as *mut _;

        self.refresh_timer();
        unsafe { self.switch_threads(prev, next) };
    }

    // FIXME?: assumes that there is at least one active thread.
    pub fn switch_to_next(&self) {
        unsafe { ::interrupts::lock_on_cpu() };
        debug_assert!(self.get().len() >= 1);
        if self.get().len() > 1 {
            let th = self.get().pop_front().unwrap();
            let prev = th.as_ref() as *const _ as *mut _;
            self.get().push_back(th);
            let next = self.get().front().unwrap().as_ref() as *const _ as *mut _;

            self.refresh_timer();
            unsafe { self.switch_threads(prev, next) };
        } else {
            unsafe { ::interrupts::unlock_on_cpu() };
        }
    }

    fn refresh_timer(&self) {
        unsafe { pit::start_periodical(TIME_FRAME) };
    }

    /// It's inside `Scheduler` to ensure thread safety
    unsafe fn switch_threads(&self, prev: *mut ThreadRepr, next: *mut ThreadRepr) {
        switch_thread_context(&mut (*prev).stack_ptr, (*next).stack_ptr);
    }
}


/// Should be called only by corresponding interrupt handler.
#[no_mangle]
pub unsafe fn __kernel_timer_tick() {
    ::interrupts::pic::PIC_1.end_of_interrupt();
    SCHEDULER.switch_to_next();
}


// FIXME: once
pub unsafe fn init() {
    SCHEDULER.init();
}

#[cfg(os_test)]
pub mod thread_tests {
    tests_module!("thread",
        one_spawn,
        long_computation_10_threads,
    );

    fn one_spawn() {
        let th = ::thread::spawn(|| 5);
        let res = th.join();
        assert!(res == 5);
    }

    fn long_computation_10_threads() {
        let mut threads = vec![];

        for i in 0..10 {
            let i_copy = i;
            threads.push(
                ::thread::spawn(move || {
                    let mut sum: u64 = 0;
                    for j in i..500_000 {
                        sum += j;
                    }
                    // TODO: automatically check
                    // uncomment this line to check that the order isn't straight
                    // (though it may be straight with high probability)
                    // All threads should finish at the same time (fairness)
                    //println!("thread {} finished", i);
                    sum
                })
            );
        }

        let mut res = vec![];
        for th in threads {
            res.push(th.join());
        }

        assert_eq!(res, [124999750000, 124999750000, 124999749999, 124999749997, 124999749994, 124999749990,
                            124999749985, 124999749979, 124999749972, 124999749964])
    }
}
