
prelude!();

use ::vga;
use super::pic::{PIC_1, PIC_2};

mod item;
pub use self::item::*;
mod idt_ptr;
pub use self::idt_ptr::*;

extern "C" {
    fn interrupt0();
    fn interrupt1();
}

extern {
    fn __kernel_timer_tick();
}

const IDT_SIZE: usize = 64;

static mut IDT_TABLE: [IdtItem; IDT_SIZE] = [IdtItem::new(); IDT_SIZE];

#[no_mangle]
pub unsafe extern "C" fn handle_interrupt(num: u8, error_code: u64) {
    // thread timer
    if num == PIC_1.get_interrupt_idt_num(0) {
        __kernel_timer_tick();
        // it will send EOI itself
        return;
    }

    vga::print(b"!! Interrupt !!");
    println!("!! Interrupt: {:#x}, Error code: {:#x}", num, error_code);
    if PIC_1.has_interrupt(num) {
        PIC_1.end_of_interrupt();
    }
    if PIC_2.has_interrupt(num) {
        PIC_1.end_of_interrupt();
        PIC_2.end_of_interrupt();
    }
}

pub unsafe fn init() {
    let diff = interrupt1 as usize - interrupt0 as usize;
    for i in 0..IDT_SIZE {
        IDT_TABLE[i].set_offset(interrupt0 as usize + diff * i);
    }

    // 0 pic interrupt is used by thread scheduler
//    { // FIXME
//        let idx = PIC_1.get_interrupt_idt_num(0) as usize;
//        IDT_TABLE[idx].type_attr = InterruptType::ValidInterruptGate;
//    }

    let ptr = IdtPtr::new(&IDT_TABLE);
    ptr.load()
}
