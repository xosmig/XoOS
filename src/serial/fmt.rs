
// FIXME: it just doesn't work!!

//use super::Serial;
#[allow(unused)]
use super::super::core::fmt::{self, Write};
#[allow(unused)]
use super::super::serial::Serial;

struct Wrapper<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Wrapper<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Wrapper {
            buf: buf,
            offset: 0,
        }
    }
}

impl<'a> fmt::Write for Wrapper<'a> {
    #[allow(unreachable_code)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        panic!(); // this place is never reached
        Ok(())
    }
}

struct Output;

impl fmt::Display for Output {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // this is place is never reached
        Serial::get().write_str(b"There is a panic here:\n");
        panic!();
    }
}

pub fn test() {
    let x = 123;
    let mut buf = [b'0'; 20];

    // write!(Wrapper::new(&mut buf), "{}", x).expect("Can't write");
    {
        let mut wrapper = Wrapper::new(&mut buf);
        // it's just do nothing!
        fmt::write(
            &mut wrapper,
            format_args!("{} should be panic: {}", x, Output)
        ).expect("Can't write");
    }

    // assert_eq!(&buf[..3], &b"123"[..]); // it's a dream
    assert_eq!(&buf[..3], &b"000"[..]); // it's a reality

    if buf[0] == b'0' {
        Serial::get().write_str(b"\n[T_T]\n\n");
    }
}
