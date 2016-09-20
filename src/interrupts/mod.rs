
pub mod idt;
mod pic;

// Num must be known at the compile time. Then we need a macro.
macro_rules! interrupt {
    ($num: expr) => (
        asm!(concat!("INT ",stringify!($num)) : /*out*/ :  : /*clobbers*/ : "volatile", "intel")
    );
}

pub use self::pic::*;
