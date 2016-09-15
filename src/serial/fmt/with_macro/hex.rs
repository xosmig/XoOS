
use super::super::super::Serial;
use super::print::Print;

const DIGITS: [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                          b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];

pub trait PrintHex {
    fn print_hex(&self);
}

pub struct Hex<'a, T: PrintHex + 'a> {
    obj: &'a T
}

impl<'a, T: PrintHex + 'a> Print for Hex<'a, T> {
    fn print(&self) {
        self.obj.print_hex();
    }
}

pub fn hex<'a, T: PrintHex>(obj: &'a T) -> Hex<'a, T> {
    Hex { obj: obj }
}

impl PrintHex for u64 {
    fn print_hex(&self) {
        const BUF_SIZE: usize = 20;

        let mut len = 0;
        let mut x = *self;
        let mut buf = [0; BUF_SIZE];

        loop {
            len += 1;

            buf[BUF_SIZE - len] = DIGITS[(x & 0b1111) as usize];
            x >>= 4;

            if x == 0 {
                break;
            }
        }

        Serial::get().write_str(&buf[(BUF_SIZE - len)..]);
    }
}

impl PrintHex for i64 {
    fn print_hex(&self) {
        let mut x = *self;
        if x < 0 {
            Serial::get().write_byte(b'-');
            x = -x;
        }
        (x as u64).print_hex();
    }
}

generate_by_cast!(Trait: PrintHex; Fn: print_hex; Target: u64; Items: u32, u16, u8);
generate_by_cast!(Trait: PrintHex; Fn: print_hex; Target: i64; Items: i32, i16, i8);

generate_for_arrays!(
        Trait: PrintHex; Fn: print_hex;
        Sizes: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
               17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    );

