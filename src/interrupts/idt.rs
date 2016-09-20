use ::vga;
use ::fmt::Write;
use ::core::mem::size_of;

extern {
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
    pub fn get_offset(&self) -> u64 {
        self.offset1 as u64 | ((self.offset2 as u64) << 16) as u64 | ((self.offset3 as u64) << 32)
    }

    pub unsafe fn set_offset(&mut self, offset: *const ()) {
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
    fn new(data: &[IdtItem]) -> Self {
        IdtPtr { limit: (data.len() * size_of::<IdtItem>() - 1) as u16, base: data.as_ptr() }
    }

    unsafe fn load(&self) {
        asm!("lidt (%rdi)" : /*out*/ : "{rdi}"(self as *const IdtPtr) : /*clb*/ : "volatile");
    }
}

const IDT_SIZE: usize = 64;

static mut IDT_TABLE: [IdtItem; IDT_SIZE] = [IdtItem {
    offset1: 0,                 // offset bits 0..15
    selector: 0x08,             // a code segment selector in GDT or LDT
    zero1: 0,                   // something unused
    type_attr: 0b1_000_1111,    // valid_unknown_type(trap gate)
    offset2: 0,                 // offset bits 16..31
    offset3: 0,                 // offset bits 32..63
    zero2: 0,                   // reserved
}; IDT_SIZE];

#[no_mangle]
#[allow(private_no_mangle_fns)]
pub unsafe extern "C" fn handle_interrupt(num: u8, error_code: u16) {
    vga::print(b"!! Interrupt !!");
    println!("!! Interrupt: {}", num);
    loop {}
}

#[no_mangle]
pub unsafe fn mysetup() {
    // FIXME
    let tmp = interrupt1 as *const ();
    assert!(tmp == 0 as *const ());
    assert!(interrupt1 as *const () != 0 as *const ());

//    IDT_TABLE[0].set_offset(interrupt1 as *const ());
//    interrupt1();
//    for i in 0..IDT_SIZE {
//        IDT_TABLE[i].set_offset(interrupt1 as *const ());
//    }
//    let ptr = IdtPtr::new(&IDT_TABLE);
//    ptr.load()
}

pub mod tests {
    // TODO
}
