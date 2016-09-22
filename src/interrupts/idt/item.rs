#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct IdtItem {
    offset1: u16,     // offset bits 0..15
    selector: u16,    // a code segment selector in GDT or LDT
    zero1: u8,        // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    type_attr: u8,    // type and attributes
    offset2: u16,     // offset bits 16..31
    offset3: u32,     // offset bits 32..63
    zero2: u32,       // reserved
}

impl IdtItem {
    pub const fn new() -> Self {
        IdtItem {
            offset1: 0,                 // offset bits 0..15
            selector: 0x08,             // a code segment selector in GDT or LDT
            zero1: 0,                   // something unused
            type_attr: 0b1_000_1111,    // valid_unknown_type(trap gate)
            offset2: 0,                 // offset bits 16..31
            offset3: 0,                 // offset bits 32..63
            zero2: 0,                   // reserved
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset1 as usize | ((self.offset2 as usize) << 16) | ((self.offset3 as usize) << 32)
    }

    pub unsafe fn set_offset(&mut self, offset: usize) {
        self.offset1 = offset as u16;
        self.offset2 = (offset >> 16) as u16;
        self.offset3 = (offset >> 32) as u32;
    }
}
