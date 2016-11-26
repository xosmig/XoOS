
pub use ::core::sync::*;

pub mod not_owning;

mod generic_mutex;
pub use self::generic_mutex::*;

//pub use self::spinlock::*;
//pub use self::lock_guard::*;


pub type SpinMutex<T> = GMutex<T, not_owning::SpinLock>;
