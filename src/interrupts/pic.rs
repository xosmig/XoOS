
pub fn lock_on_cpu() {
    unsafe { asm!("cli" : /*out*/ : /*in*/ : /*clb*/ : "volatile" ) };
}

pub fn unlock_on_cpu(){
    unsafe { asm!("sti" : /*out*/ : /*in*/ : /*clb*/ : "volatile" ) };
}

//static mut MASK: u8 = 0;

//pub fn lock(num: u8) {
//     port;
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
//pub fn lock_all() {
//
//}
//
//pub fn unlock_all() {
//
//}
