
//! note that you have to add `use ::fmt::Write` to get macro works

// FIXME: a really horrible bug!!
// FIXME: can't format number system ("{:o}"/"{:x}")
// FIXME: write!(anything, "{:o}", 123) leads to fail

pub use ::core::fmt::*;

#[derive(Debug)]
pub struct Buffer<'a> {
    data: &'a mut [u8],
    length: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Buffer {
            data: buf,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl<'a> Write for Buffer<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        let bytes = s.as_bytes();

        let tail = &mut self.data[self.length..];
        let tail = &mut tail[..bytes.len()];
        if tail.len() >= bytes.len() {
            tail.copy_from_slice(bytes);
            Ok(())
        } else {
            Err(Error)
        }
    }

    fn write_char(&mut self, c: char) -> Result {
        if self.len() < self.capacity() {
            self.data[self.length] = c as u8;
            self.length += 1;
            Ok(())
        } else {
            Err(Error)
        }
    }
}

/// prints data to the serial port
macro_rules! print {
    ($fmt: expr) => (
        write!(serial::Serial::get(), $fmt).expect("`print!` or `println` failed")
    );
    ($fmt: expr, $( $arg: expr ),* ) => (
        write!(serial::Serial::get(), $fmt $( ,$arg )* ).expect("`print!` or `println!` failed")
    );
}

/// prints data to the serial port
macro_rules! println {
    ($fmt: expr) => (
        print!(concat!($fmt, "\n")).expect
    );
    ($fmt: expr, $( $arg: expr ),* ) => (
        print!(concat!($fmt, "\n") $( ,$arg )* )
    );
}

/// gets a byte array for buffer as the first parameter
/// and writes formatted data into it
/// returns the number of written bytes
macro_rules! format {
    ($buf: expr, $fmt: expr, $( $arg: expr ),*) => ({
        let mut buffer = ::fmt::Buffer::new(&mut $buf);
        write!(buffer, $fmt $( ,$arg )*);
        buffer.len()
    });
}
