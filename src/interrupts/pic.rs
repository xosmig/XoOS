
const PIC1_COMMAND_PORT: u16 = 0x20;
const PIC1_DATA_PORT: u16 = 0x21;
const PIC1_IDT_START: u8 = 32;

const PIC2_COMMAND_PORT: u16 = 0xA0;
const PIC2_DATA_PORT: u16 = 0xA1;
const PIC2_IDT_START: u8 = 40;

use ::ioports::*;

pub unsafe fn init() {
    // initialization command
    write::<u8>(PIC1_COMMAND_PORT, 0x11);
    write::<u8>(PIC2_COMMAND_PORT, 0x11);

    write::<u8>(PIC1_DATA_PORT, PIC1_IDT_START);
    write::<u8>(PIC2_DATA_PORT, PIC2_IDT_START);

    // the slave pic on the second place
    write::<u8>(PIC1_DATA_PORT, 0b0000_0100);
    write::<u8>(PIC2_DATA_PORT, 2);

    // just some not interesting parameters
    write::<u8>(PIC1_DATA_PORT, 0x01);
    write::<u8>(PIC2_DATA_PORT, 0x01);

    // set empty interrupt masks
    write::<u8>(PIC1_DATA_PORT, 0);
    write::<u8>(PIC2_DATA_PORT, 0);
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
        write::<u8>(PIC1_DATA_PORT, 0xFF);
        write::<u8>(PIC2_DATA_PORT, 0xFF);
    }
}

//pub fn unlock_all() {
//
//}
