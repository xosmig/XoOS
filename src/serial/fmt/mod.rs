
mod with_macro;

pub use self::with_macro::*;

macro_rules! print {
    ( $( $x:expr ),* ) => {
        {
            $(
                $x.print();
            )*
        }
    };
}
