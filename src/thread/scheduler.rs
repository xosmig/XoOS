
prelude!();

use ::alloc::arc::Arc;
use ::collections::LinkedList;
use super::{ ThreadRepr, refresh_timer };
use super::int_guard::*;

struct Scheduler {
    // TODO: inplace queue
    queue: LinkedList<Arc<ThreadRepr>>,
}


impl Scheduler {
    pub fn add(&mut self, thread: Arc<ThreadRepr>) {
        let guard = IntGuard::new();
        self.queue.push_back(thread);
    }

    /// Removes current thread from queue.
    pub fn sleep_current(&mut self) {
        let guard = IntGuard::new();
        let th = self.queue.pop_front();
        refresh_timer();
//        switch_threads(th, self.queue.front());
    }

    pub fn next(&mut self) {
        let guard = IntGuard::new();
//        switch_threads(self.queue.front());
    }
}
