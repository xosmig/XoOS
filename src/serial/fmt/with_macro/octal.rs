
use super::super::super::Serial;
use super::print::Print;
use super::{BUF, BUF_SIZE};

pub trait PrintOctal {
    fn print_octal(&self);
}

pub struct Octal<'a, T: PrintOctal + 'a> {
    obj: &'a T
}

impl<'a, T: PrintOctal + 'a> Print for Octal<'a, T> {
    fn print(&self) {
        self.obj.print_octal();
    }
}

pub fn octal<'a, T: PrintOctal>(obj: &'a T) -> Octal<'a, T> {
    Octal { obj: obj }
}

impl PrintOctal for u64 {
    fn print_octal(&self) {
        let mut len = 0;
        let mut x = *self;

        loop {
            len += 1;

            unsafe { BUF[BUF_SIZE - len] = (x & 0b111) as u8 + b'0' };
            x >>= 3;

            if x == 0 {
                break;
            }
        }

        unsafe { Serial::get().write_str(&BUF[(BUF_SIZE - len)..]) };
    }
}

impl PrintOctal for i64 {
    fn print_octal(&self) {
        let mut x = *self;
        if x < 0 {
            Serial::get().write_byte(b'-');
            x = -x;
        }
        (x as u64).print_octal();
    }
}

generate_by_cast!(Trait: PrintOctal; Fn: print_octal; Target: u64; Items: u32, u16, u8);
generate_by_cast!(Trait: PrintOctal; Fn: print_octal; Target: i64; Items: i32, i16, i8);

generate_for_arrays!(
        Trait: PrintOctal; Fn: print_octal;
        Sizes: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );
