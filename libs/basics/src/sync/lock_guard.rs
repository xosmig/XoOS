
use super::*;
use ::core::marker::PhantomData;

/// Simple guard.
pub struct LockGuard<'a, L: 'a + Lock> {
    lock: &'a L,
}

impl<'a, L: 'a + Lock> Guard<'a> for LockGuard<'a, L> {
    type LockT = L;

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


pub struct Mutex<'a, L: Lock> {
    lock: L,
    phantom: PhantomData<&'a ()>,
}

impl<'a, L: Lock + 'a> OwningLock<'a> for Mutex<'a, L> {
    type GuardT = LockGuard<'a, L>;

    fn lock(&'a self) -> LockGuard<'a, L> {
        LockGuard::new(&self.lock)
    }
}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> { }
