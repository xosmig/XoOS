
prelude!();

use ::sync::SpinLock;
use ::sync::Lock;

pub struct IntGuard {}

static LOCK: SpinLock = SpinLock::new();

impl IntGuard {
    pub fn new() -> Self {
        LOCK.acquire();
        unsafe { ::interrupts::lock_on_cpu() };
        IntGuard {}
    }
}

impl Drop for IntGuard {
    fn drop(&mut self) {
        unsafe { ::interrupts::unlock_on_cpu() };
        LOCK.release();
    }
}
