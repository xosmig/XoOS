
prelude!();

use ::core::sync::atomic::*;
use super::Lock;

pub struct SpinLock {
    current: AtomicUsize,
    next: AtomicUsize,
}

impl SpinLock {
    pub const fn new() -> Self {
         SpinLock {
             current: AtomicUsize::new(0),
             next: AtomicUsize::new(0),
         }
    }
}

impl Lock for SpinLock {
    fn acquire(&self) {
        let ticket = self.next.fetch_add(1, Ordering::Relaxed);
        while self.current.load(Ordering::Acquire) != ticket {}
    }
    fn release(&self) {
        self.current.fetch_add(1, Ordering::Release);
    }
}
