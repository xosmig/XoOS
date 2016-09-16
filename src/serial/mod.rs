//! contains class Serial - singleton for writing symbols to serial port.
//! also contains the definition of `print!` and `println!`

use ::fmt;

use ::ioport;
use ::utility::*;

const PORT: u16 = 0x3f8;

pub struct Serial;

static mut OBJECT: Serial = Serial {  };

impl Serial {
    pub fn get() -> &'static mut Self {
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                // Disable all interrupts
                ioport::write::<u8>(PORT + 1, 0b0000_0000);
                // Enable DLAB (set baud rate divisor)
                ioport::write::<u8>(PORT + 3, 0b1000_0000);
                // Set divisor to 3 (lo byte) 38400 baud
                ioport::write::<u8>(PORT + 0, 0b0000_0011);
                //                  (hi byte)
                ioport::write::<u8>(PORT + 1, 0b0000_0000);
                // Frame format: 8 bits, no parity, one stop bit
                ioport::write::<u8>(PORT + 3, 0b0000_0011);
                INIT = true;
            }
        }

        unsafe { &mut OBJECT }
    }
}

impl fmt::Write for Serial {
    fn write_char(&mut self, c: char) -> fmt::Result {
        loop {
            let free = unsafe { ioport::read::<u8>(PORT + 5) & bit::<u8>(5) };
            if free != 0 {
                unsafe { ioport::write(PORT + 0, c as u8) };
                break;
            }
        }
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.chars() {
            self.write_char(char);
        }
        Ok(())
    }
}
