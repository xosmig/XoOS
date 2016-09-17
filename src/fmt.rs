//! note that you have to add `use ::fmt::Write` to get macro works

pub use ::core::fmt::*;

/// stringstream on a custom buffer
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
        if tail.len() >= bytes.len() {
            self.length += bytes.len();
            let tail = &mut tail[..bytes.len()];
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
        write!(::serial::Serial::get(), $fmt).expect("`print!` or `println` failed")
    );
    ($fmt: expr, $( $arg: expr ),* ) => (
        write!(::serial::Serial::get(), $fmt $( ,$arg )* ).expect("`print!` or `println!` failed")
    );
}

/// prints data to the serial port
macro_rules! println {
    ($fmt: expr) => (
        print!(concat!($fmt, "\n"))
    );
    ($fmt: expr, $( $arg: expr ),* ) => (
        print!(concat!($fmt, "\n") $( ,$arg )* )
    );
}

/// gets a byte array for buffer as the first parameter
/// and writes formatted data into it
/// returns pair: (???, the number of written bytes)
macro_rules! format {
    ($data: expr, $fmt: expr) => ({
        let mut buf = ::fmt::Buffer::new(&mut $data);
        write!(buf, $fmt).expect("`format!` failed");
        buf.len()
    });
    ($data: expr, $fmt: expr, $( $arg: expr ),*) => ({
        let mut buf = ::fmt::Buffer::new(&mut $data);
        write!(buf, $fmt $( ,$arg )*).expect("`format!` failed");
        buf.len()
    });
}

#[cfg(os_test)]
pub mod tests {
    use super::*;

    pub fn all() {
        println!("Fmt tests... running");
        overflow();
        numbers();
        println!("Fmt tests... OK");
    }

    fn overflow() {
        let mut data = [0; 10];
        let mut buf = Buffer::new(&mut data);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.capacity(), 10);
        write!(buf, "123{}", 4).unwrap();
        assert_eq!(buf.len(), 4);
        write!(buf, "{}", "5678").unwrap();
        assert_eq!(buf.len(), 8);
        let err = write!(buf, "{}", "9ab");
        assert!(err.is_err());
        assert_eq!(buf.len(), 8);
    }

    fn numbers() {
        let mut data = [0; 32];
        let mut x: i64 = 123456789;
        format!(data, "{}", x);
        assert!(data.starts_with(b"123456789"));
        format!(data, "{:#o}", x);
        assert!(data.starts_with(b"0o726746425"));
        format!(data, "{:#X}", x);
        assert!(data.starts_with(b"0x75BCD15"));
    }
}
