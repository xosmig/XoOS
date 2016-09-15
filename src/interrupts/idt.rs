
#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy)]
struct IdtItem {
    data: [u32; 16]
}

impl IdtItem {
    // TODO
}

static IDT_TABLE: [IdtItem; 256] = [IdtItem { data: [0; 16] }; 256];
