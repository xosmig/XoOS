
pub use ::core::sync::*;

pub mod not_owning;

mod generic_mutex;
pub use self::generic_mutex::*;


pub type SpinMutexGuard<'a, T> = GMutexGuard<'a, T, not_owning::SpinLock>;
pub type SpinMutex<T> = GMutex<T, not_owning::SpinLock>;
