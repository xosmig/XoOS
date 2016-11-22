
pub use ::core::sync::*;

mod ticket_lock;

pub use self::ticket_lock::*;

pub trait Lock {
    fn acquire(&mut self);
    fn release(&mut self);
}

pub struct LockGuard<'a, L: 'a + Lock> {
    lock: &'a mut L,
}

impl<'a, L: 'a + Lock> LockGuard<'a, L> {
    fn new(lock: &'a mut L) -> Self {
        lock.acquire();
        LockGuard { lock: lock }
    }
}

impl<'a, L: 'a + Lock> Drop for LockGuard<'a, L> {
    fn drop(&mut self) {
        self.lock.release();
    }
}
