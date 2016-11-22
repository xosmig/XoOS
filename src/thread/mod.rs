
prelude!();

mod int_guard;
mod pit;
mod scheduler;

use ::alloc::arc::Arc;
use ::collections::string::String;
use ::sync::{ SpinLock, LockGuard };
use ::core::marker::PhantomData;
use self::int_guard::IntGuard;


const TIME_UNIT: u16 = 256;
const TIME_UNIT_CNT: u16 = 16;


#[repr(C)]
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
}


struct ThreadRepr {
    name: Option<String>,
    context: Context,
}


pub struct Thread {
    repr: Arc<ThreadRepr>,
}

pub struct JoinHandle<T> {
    thread: Thread,
    phantom: PhantomData<T>,
}

impl<T> JoinHandle<T> {
    // TODO?: panic handling
    pub fn join() -> T /*Result<T>*/ {
        // TODO
        unsafe { mem::uninitialized() }
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        // TODO
    }
}

pub fn sleep() {
    // TODO
}

pub fn spawn<T, F>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
{
    // TODO
    unsafe { mem::uninitialized() }
}



// TODO: once
pub unsafe fn init() {
    refresh_timer();
}

/// Should be called only by corresponding interruption handler.
#[no_mangle]
pub unsafe fn __kernel_timer_tick() {

}

fn refresh_timer() {
    unsafe { pit::start_periodical(TIME_UNIT) };
}

#[naked]
fn switch_threads_impl() {
    
}
