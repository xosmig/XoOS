use ::core::mem::size_of;

use super::item::*;

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
