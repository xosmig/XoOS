
// TODO: refactor it! Should be `struct Pic`

use ::ioports::*;

static mut PIC1_COMMAND: IOPort<u8, u8> = IOPort::new(0x20);
static mut PIC1_DATA: IOPort<u8, u8> = IOPort::new(0x21);
const PIC1_IDT_START: u8 = 32;

static mut PIC2_COMMAND: IOPort<u8, u8> = IOPort::new(0xA0);
static mut PIC2_DATA: IOPort<u8, u8> = IOPort::new(0xA1);
const PIC2_IDT_START: u8 = 40;

pub unsafe fn init() {
    // initialization command
    PIC1_COMMAND.write(0x11);
    PIC2_COMMAND.write(0x11);

    PIC1_DATA.write(PIC1_IDT_START);
    PIC2_DATA.write(PIC2_IDT_START);

    // the slave pic on the second place
    PIC1_DATA.write(0b0000_0100);
    PIC2_DATA.write(2);

    // just some not interesting parameters
    PIC1_DATA.write(0x01);
    PIC2_DATA.write(0x01);

    // clear interrupt masks
    PIC1_DATA.write(0);
    PIC2_DATA.write(0);
}

//pub fn lock(num: u8) {
//    port;
//    uint8_t value;
//
//    if(IRQline < 8) {
//        port = PIC1_DATA;
//    } else {
//        port = PIC2_DATA;
//        IRQline -= 8;
//    }
//    value = inb(port) | (1 << IRQline);
//    outb(port, value);
//}

//pub fn unlock(num: u8) {
//
//}
//

pub fn lock_all() {
    unsafe {
        PIC1_DATA.write(0xFF);
        PIC2_DATA.write(0xFF);
    }
}

//pub fn unlock_all() {
//
//}
