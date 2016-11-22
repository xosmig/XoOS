
use ::core::sync::atomic::*;
use super::Lock;

struct TicketLock {
    current: AtomicUsize,
    next: AtomicUsize,
}

impl TicketLock {
    pub const fn new() -> Self {
         TicketLock { current: AtomicUsize::new(0), next: AtomicUsize::new(0) }
    }
}

impl Lock for TicketLock {
    fn acquire(&mut self) {
        let ticket = self.next.fetch_add(1, Ordering::Relaxed);
        while self.current.load(Ordering::Acquire) != ticket {}
    }
    fn release(&mut self) {
        self.current.fetch_add(1, Ordering::Release);
    }
}
