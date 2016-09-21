
pub mod idt;
pub mod pic;

// FIXME: I can't make a normal function
macro_rules! interrupt {
    ($num: expr) => (
        asm!(concat!("INT ",stringify!($num)) : /*out*/ :  : /*clobbers*/ : "volatile", "intel")
    );
}

pub fn lock_on_cpu() {
    unsafe { asm!("cli" : /*out*/ : /*in*/ : /*clb*/ : "volatile" ) };
}

pub fn unlock_on_cpu(){
    unsafe { asm!("sti" : /*out*/ : /*in*/ : /*clb*/ : "volatile" ) };
}
