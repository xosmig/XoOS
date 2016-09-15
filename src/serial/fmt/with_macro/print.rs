
use super::super::super::Serial;

pub trait Print {
    fn print(&self);
}

impl Print for u64 {
    fn print(&self) {
        const BUF_SIZE: usize = 20;

        let mut len = 0;
        let mut x = *self;
        let mut buf = [0; BUF_SIZE];

        loop {
            len += 1;

            buf[BUF_SIZE - len] = (x % 10) as u8 + b'0';
            x /= 10;

            if x == 0 {
                break;
            }
        }

        Serial::get().write_str(&buf[(BUF_SIZE - len)..]);
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

generate_by_cast!(Trait: Print; Fn: print; Target: u64; Items: u32, u16, u8);
generate_by_cast!(Trait: Print; Fn: print; Target: i64; Items: i32, i16, i8);

generate_for_arrays!(
        Trait: Print; Fn: print;
        Sizes: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
