
mod print;
#[macro_use] mod octal;
mod hex;

const BUF_SIZE: usize = 1024;
static mut BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];

pub use self::print::*;
pub use self::octal::octal;
//pub use self::hex::;

macro_rules! print {
    ( $( $x:expr ),* ) => {
        {
            $(
                $x.print();
            )*
        }
    };
}
