
mod idt;
mod masking;

macro_rules! interrupt {
    ($num: expr) => (
        asm!(concat!("INT ",stringify!($num)) : /*out*/ :  : /*clobbers*/ : "volatile", "intel");
    );
}

pub use self::masking::*;
