
use ::ioports::*;
use ::interrupts::pic;

static mut COMMAND_PORT: IOPort<(), u8> = IOPort::new(0x43);
static mut DATA_PORT: IOPort<(), u8> = IOPort::new(0x40);

pub fn run(init: u16) {
    unsafe {
        // (channel: 0)_(initial bytes: both)_(mode: 2(periodical))_(bcd: no)
        COMMAND_PORT.write(0b00_11_010_0);
        DATA_PORT.write(init as u8); // lo byte
        DATA_PORT.write((init >> 8) as u8); // hi byte
    }
}

pub fn unlock_interrupt() {
    pic::unlock(0);
}

//pub fn lock_interrupt() { // TODO
//}
