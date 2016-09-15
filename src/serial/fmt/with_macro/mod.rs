#[macro_use]
mod macros;

mod print;
mod octal;
mod hex;

pub use self::print::Print;
pub use self::octal::octal;
pub use self::hex::hex;
