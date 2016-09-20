use ::vga;
use ::fmt::Write;
use ::core::mem::size_of;

// macros cannot expand to foreign items :(
extern "C" {
    fn interrupt0();
    fn interrupt1();
}

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

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct IdtPtr {
    limit: u16,
    base: *const IdtItem,
}

impl IdtPtr {
    pub fn new(data: &[IdtItem]) -> Self {
        IdtPtr { limit: (data.len() * size_of::<IdtItem>() - 1) as u16, base: data.as_ptr() }
    }

    pub unsafe fn load(&self) {
        asm!("lidt (%rdi)" : /*out*/ : "{rdi}"(self as *const IdtPtr) : /*clb*/ : "volatile");
    }
}

const IDT_SIZE: usize = 64;

static mut IDT_TABLE: [IdtItem; IDT_SIZE] = [IdtItem::new(); IDT_SIZE];

#[allow(unused)] // FIXME
#[no_mangle]
pub unsafe extern "C" fn handle_interrupt(num: u8, error_code: u16) {
    vga::print(b"!! Interrupt !!");
    println!("!! Interrupt: {}", num);
}

pub unsafe fn setup() {
    let diff = interrupt1 as usize - interrupt0 as usize;
    for i in 0..IDT_SIZE {
        IDT_TABLE[i].set_offset(interrupt0 as usize + diff * i);
    }
    let ptr = IdtPtr::new(&IDT_TABLE);
    ptr.load()
}

pub mod tests {
    // TODO
}
