
pub use ::core::sync::*;

mod spinlock;

pub use self::spinlock::*;

pub trait Lock {
    fn acquire(&self);
    fn release(&self);
}

pub struct LockGuard<'a, L: 'a + Lock> {
    lock: &'a L,
}

impl<'a, L: 'a + Lock> LockGuard<'a, L> {
    fn new(lock: &'a L) -> Self {
        lock.acquire();
        LockGuard { lock: lock }
    }
}

impl<'a, L: 'a + Lock> Drop for LockGuard<'a, L> {
    fn drop(&mut self) {
        self.lock.release();
    }
}
