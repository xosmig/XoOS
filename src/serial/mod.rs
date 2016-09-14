
pub mod fmt;

use super::ioport;
use super::utility::*;

const PORT: u16 = 0x3f8;

pub struct Serial;

static OBJECT: Serial = Serial {  };
static mut INIT: bool = false;

impl Serial {
    pub fn get() -> &'static Self {
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
        &OBJECT
    }

    pub fn write_byte(&self, byte: u8) {
        loop {
            let free = unsafe { ioport::read::<u8>(PORT + 5) & bit::<u8>(5) };
            if free != 0 {
                unsafe { ioport::write(PORT + 0, byte) };
                break;
            }
        }
    }

    pub fn write_str(&self, data: &[u8]) {
        for byte in data {
            self.write_byte(*byte);
        }
    }
}
