
use super::super::Serial;
use super::*;
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
