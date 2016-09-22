
use ::ioports::*;

static mut COMMON_CMD: IOPort<u8, u8> = IOPort::new(0x43);
static mut TIMER_CMD: IOPort<u8, u8> = IOPort::new(0x40);


