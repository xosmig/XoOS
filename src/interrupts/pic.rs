
// FIXME: I think, this module should be refactored
// FIXME: Master pic should know about slave or slave about master?

pub struct Pic {
    command: IOPort<(), u8>,
    data: IOPort<u8, u8>,
    idt_start: u8,
}

impl Pic {
    pub unsafe fn end_of_interrupt(&mut self) {
        // undirected
        self.command.write(::utility::bit(5));
    }

    pub fn has_interrupt(&self, num: u8) -> bool {
        num >= self.idt_start && num < self.idt_start + 8
    }
}

use ::ioports::*;

pub static mut PIC_1: Pic = Pic {
    command: IOPort::new(0x20),
    data: IOPort::new(0x21),
    idt_start: 32,
};

pub static mut PIC_2: Pic = Pic {
    command: IOPort::new(0xA0),
    data: IOPort::new(0xA1),
    idt_start: 40,
};

pub unsafe fn init_default() {
    // initialization command
    PIC_1.command.write(0x11);
    PIC_2.command.write(0x11);

    PIC_1.data.write(PIC_1.idt_start);
    PIC_2.data.write(PIC_2.idt_start);

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

pub fn unlock(mut num: u8) {
    unsafe {
        let pic = if num < 8 {
            &mut PIC_1
        } else {
            num -= 8;
            &mut PIC_2
        };
        let old_mask = pic.data.read();
        pic.data.write(old_mask & !(1 << num));
    }
}

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
