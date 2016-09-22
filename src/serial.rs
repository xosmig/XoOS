//! contains class Serial - singleton for writing symbols to serial port.
//! also contains the definition of `print!` and `println!`

use ::fmt;
use ::ioports::*;

const PORT: u16 = 0x3f8;

static mut PORT_0: IOPort<(), u8> = IOPort::new(PORT + 0);
static mut PORT_1: IOPort<(), u8> = IOPort::new(PORT + 1);
// static mut PORT_2: IOPort<(), u8> = IOPort::new(PORT + 2); // unused
static mut PORT_3: IOPort<(), u8> = IOPort::new(PORT + 3);
//static mut PORT_4: IOPort<(), u8> = IOPort::new(PORT + 4); // unused
static mut PORT_5: IOPort<u8, ()> = IOPort::new(PORT + 5);

pub struct Serial;

static mut OBJECT: Serial = Serial {  };

impl Serial {
    pub fn get() -> &'static mut Self {
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                // Disable all interrupts
                PORT_1.write(0b0000_0000);
                // Enable DLAB (set baud rate divisor)
                PORT_3.write(0b1000_0000);
                // Set divisor to 3 (lo byte) 38400 baud
                PORT_0.write(0b0000_0011);
                //                  (hi byte)
                PORT_1.write(0b0000_0000);
                // Frame format: 8 bits, no parity, one stop bit
                PORT_3.write(0b0000_0011);

                INIT = true;
            }
        }

        unsafe { &mut OBJECT }
    }
}

impl fmt::Write for Serial {
    fn write_char(&mut self, c: char) -> fmt::Result {
        loop {
            let free = unsafe { PORT_5.read() & ::utility::bit(5) };
            if free != 0 {
                unsafe { PORT_0.write(c as u8) };
                break;
            }
        }
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.chars() {
            try!(self.write_char(char));
        }
        Ok(())
    }
}
