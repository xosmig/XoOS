
///! Mutex that also locks all interrupts when locked.

prelude!();

use ::core::cell::UnsafeCell;
use ::core::ops::{ Deref, DerefMut };


pub struct IntMutex<T> {
    data: UnsafeCell<T>,
}

impl<T> IntMutex<T> {
    pub fn new(t: T) -> Self {
        IntMutex { data: UnsafeCell::new(t) }
    }

    pub fn lock(&self) -> IntGuard<T> {
        unsafe {
            println!("LOCKED");
            // must be locked before locking interrupts
//            let ret = IntGuard { spin: self.spin.lock() };
            ::interrupts::lock_on_cpu();
//            ret
            IntGuard { mutex: self }
        }
    }
}
unsafe impl<T: Send> Send for IntMutex<T> {}
unsafe impl<T: Sync> Sync for IntMutex<T> {}


pub struct IntGuard<'a, T: 'a> {
//    spin: ::spin::MutexGuard<'a, T>,
    mutex: &'a IntMutex<T>,
}

impl<'a, T> Drop for IntGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { ::interrupts::unlock_on_cpu() };
    }
}

impl<'a, T> Deref for IntGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for IntGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}
