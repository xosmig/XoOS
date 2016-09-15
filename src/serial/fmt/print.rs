
use super::super::Serial;
use super::{BUF, BUF_SIZE};

pub trait Print {
    fn print(&self);
}

impl Print for u64 {
    fn print(&self) {
        let mut len = 0;
        let mut x = *self;

        loop {
            len += 1;

            unsafe { BUF[BUF_SIZE - len] = (x % 10) as u8 + b'0' };
            x /= 10;

            if x == 0 {
                break;
            }
        }

        unsafe { Serial::get().write_str(&BUF[(BUF_SIZE - len)..]) };
    }
}

impl Print for i64 {
    fn print(&self) {
        let mut x = *self;
        if x < 0 {
            Serial::get().write_byte(b'-');
            x = -x;
        }
        (x as u64).print();
    }
}

impl<'a> Print for char {
    fn print(&self) {
        Serial::get().write_byte(*self as u8);
    }
}

impl<'a> Print for &'a [u8] {
    fn print(&self) {
        Serial::get().write_str(self);
    }
}

impl<'a> Print for &'a str {
    fn print(&self) {
        Serial::get().write_str(self.as_bytes());
    }
}

// there is no trait for integers in core.
macro_rules! cast {
    ( ($to: ty) <= $( $from: ty ),*) => (
        $(
            impl Print for $from {
                fn print(&self) {
                    (*self as $to).print();
                }
            }
        )*
    );
}

cast!((u64) <= u32, u16, u8);
cast!((i64) <= i32, i16, i8);

macro_rules! array {
    ( $( $size: expr ),* ) => (
        $(
            impl<T: Print> Print for [T; $size] {
                fn print(&self) {
                    Serial::get().write_byte(b'[');
                    let mut iter = self.into_iter();
                    iter.next().unwrap().print();
                    for x in iter {
                        Serial::get().write_str(b", ");
                        x.print();
                    }
                    Serial::get().write_byte(b']');
                }
            }
        )*
    );
}

array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
       17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
