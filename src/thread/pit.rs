//! Programmable Interval Timer
//! Generates interrupts

prelude!();

use ::ioports::*;
use ::interrupts::pic;

static mut COMMAND_PORT: IOPort<(), u8> = IOPort::new(0x43);
static mut DATA_PORT: IOPort<(), u8> = IOPort::new(0x40);

pub unsafe fn start_periodical(init: u16) {
    // (channel: 0)_(initial bytes: both)_(mode: 2(periodical))_(bcd: no)
    COMMAND_PORT.write(0b00_11_010_0);
    DATA_PORT.write(init as u8);        // lo byte
    DATA_PORT.write((init >> 8) as u8); // hi byte
}

pub fn unlock_interrupt() {
    pic::unlock_interrupt(0);
}

#[allow(unused)]
pub fn lock_interrupt() {
    pic::lock_interrupt(0);
}
