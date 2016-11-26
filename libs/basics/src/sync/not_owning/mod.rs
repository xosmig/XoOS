///! Provides C++ - like synchronization primitives which don't own data.

prelude!();

mod spinlock;

pub use self::spinlock::*;

/// Similar to standard mutexes (but not to Rust's ones)
pub trait Lock {
    fn acquire(&self);
    fn release(&self);
}

/// Similar to `unique_lock` in C++.
pub trait Guard<'a>: Drop {
    type LockT: Lock;
    fn guard(lock: &'a Self::LockT) -> Self;
}
