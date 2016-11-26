
prelude!();

use ::core::cell::UnsafeCell;

use super::not_owning::{ Lock };
use ::core::ops::{ Deref, DerefMut };


pub struct GMutexGuard<'a, T, L>
    where T: 'a, L: 'a + Lock + Default
{
    mutex: &'a GMutex<T, L>,
}

impl<'a, T, L> Deref for GMutexGuard<'a, T, L>
    where T: 'a, L: 'a + Lock + Default
{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T, L> DerefMut for GMutexGuard<'a, T, L>
    where T: 'a, L: 'a + Lock + Default
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T, L> Drop for GMutexGuard<'a, T, L>
    where T: 'a, L: 'a + Lock + Default
{
    fn drop(&mut self) {
        self.mutex.lock.release();
    }
}


#[derive(Default)]
pub struct GMutex<T, L: Lock + Default> {
    data: UnsafeCell<T>,
    lock: L,
}
unsafe impl<T: Send, L: Lock + Default> Send for GMutex<T, L> {}
unsafe impl<T: Send, L: Lock + Default> Sync for GMutex<T, L> {}

impl<T: Send, L: Lock + Default> GMutex<T, L> {
    pub fn new(data: T) -> Self {
        GMutex {
            data: UnsafeCell::new(data),
            lock: L::default(),
        }
    }

    pub fn lock(&self) -> GMutexGuard<T, L> {
        self.lock.acquire();
        GMutexGuard { mutex: self }
    }
}


use super::not_owning::SpinLock;

impl<T: Send> GMutex<T, SpinLock> {
    pub const fn const_new(data: T) -> Self {
        GMutex {
            data: UnsafeCell::new(data),
            lock: SpinLock::new(),
        }
    }
}
