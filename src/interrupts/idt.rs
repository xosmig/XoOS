use ::vga;
use ::fmt::Write;
use ::core::mem::size_of;

extern "C" {
    fn interrupt1();
}

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct IdtItem {
    offset1: u16,     // offset bits 0..15
    selector: u16,    // a code segment selector in GDT or LDT
    zero1: u8,        // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    type_attr: u8,    // type and attributes
    offset2: u16,     // offset bits 16..31
    offset3: u32,     // offset bits 32..63
    zero2: u32,       // reserved
}

impl IdtItem {
    const fn new() -> Self {
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

    fn get_offset(&self) -> usize {
        self.offset1 as usize | ((self.offset2 as usize) << 16) | ((self.offset3 as usize) << 32)
    }

    // FIXME: m.b. there is a normal function pointer type
    unsafe fn set_offset(&mut self, ptr: *const ()) {
        let offset = ptr as usize;

        self.offset1 = offset as u16;
        self.offset2 = (offset >> 16) as u16;
        self.offset3 = (offset >> 32) as u32;
    }
}

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct IdtPtr {
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

static mut IDT_TABLE: [IdtItem; IDT_SIZE] = [IdtItem::new(); IDT_SIZE];

#[no_mangle]
pub unsafe extern "C" fn handle_interrupt(num: u8, error_code: u16) {
    vga::print(b"!! Interrupt !!");
    println!("!! Interrupt: {}", num);
        loop {}
}

#[allow(private_no_mangle_fns)]
#[no_mangle]
pub unsafe fn mysetup() {
    // FIXME: one more strange bug

//    interrupt1();
    assert!(interrupt1 as *const () != 0 as *const ());


    let tmp = interrupt1 as *const ();
    assert!(tmp == 0 as *const ());

    assert!(interrupt1 as usize == 0);

    fn foo(ptr: *const ()) {
        assert!(ptr == 0 as *const ());
    }
    foo(interrupt1 as *const ());

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
