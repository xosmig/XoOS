
struct Pic {
    command: IOPort<u8, u8>,
    data: IOPort<u8, u8>,
}

use ::ioports::*;

static mut PIC_1: Pic = Pic { command: IOPort::new(0x20), data: IOPort::new(0x21) };
const PIC_1_IDT_START: u8 = 32;

static mut PIC_2: Pic = Pic { command: IOPort::new(0xA0), data: IOPort::new(0xA1) };
const PIC_2_IDT_START: u8 = 40;

pub unsafe fn init_default() {
    // initialization command
    PIC_1.command.write(0x11);
    PIC_2.command.write(0x11);

    PIC_1.data.write(PIC_1_IDT_START);
    PIC_2.data.write(PIC_2_IDT_START);

    // the slave pic on the second place
    PIC_1.data.write(0b0000_0100);
    PIC_2.data.write(2);

    // just some not interesting parameters
    PIC_1.data.write(0x01);
    PIC_2.data.write(0x01);

    // clear interrupt masks
    PIC_1.data.write(0);
    PIC_2.data.write(0);
}

//pub fn lock(num: u8) { // TODO
//}

//pub fn unlock(num: u8) { // TODO
//}


pub fn lock_all() {
    unsafe {
        PIC_1.data.write(0xFF);
        PIC_2.data.write(0xFF);
    }
}

pub fn unlock_all() {
    unsafe {
        PIC_1.data.write(0x00);
        PIC_2.data.write(0x00);
    }
}
