
///! Mutex that also locks all interrupts when locked.

prelude!();

use ::core::cell::UnsafeCell;
use ::core::ops::{ Deref, DerefMut };


pub struct IntMutex<T> {
    spin: ::spin::Mutex<T>,
}

impl<T> IntMutex<T> {
    pub fn new(t: T) -> Self {
        IntMutex { spin: ::spin::Mutex::new(t) }
    }

    pub fn lock(&self) -> IntGuard<T> {
        unsafe {
            // must be locked before locking interrupts
            let ret = IntGuard { spin: self.spin.lock() };
            ::interrupts::lock_on_cpu();
            ret
        }
    }
}


pub struct IntGuard<'a, T: 'a> {
    spin: ::spin::MutexGuard<'a, T>,
}

impl<'a, T> Drop for IntGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { ::interrupts::unlock_on_cpu() };
    }
}

impl<'a, T> Deref for IntGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.spin
    }
}

impl<'a, T> DerefMut for IntGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.spin
    }
}
