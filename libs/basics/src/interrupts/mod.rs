
pub mod idt;
pub mod pic;

/// Generates interrupt with the given constant number.
/// Warning: Should be just a number. Not a variable or an expression.
macro_rules! interrupt {
    ($num: expr) => (
        asm!(concat!("INT ",stringify!($num)) : /*out*/ :  : /*clobbers*/ : "volatile", "intel")
    );
}

/// Initializes idt and pic.
/// Locks all interrupts on pic and unlocks it on a CPU.
pub unsafe fn init() {
    idt::init();
    pic::init();
    pic::lock_all();
    unlock_on_cpu();
}

/// Lock all interrupts on a CPU.
pub unsafe fn lock_on_cpu() {
    asm!("cli" : /*out*/ : /*in*/ : /*clb*/ : "volatile" );
}

/// Unlock all interrupts on a CPU.
pub unsafe fn unlock_on_cpu() {
    asm!("sti" : /*out*/ : /*in*/ : /*clb*/ : "volatile" );
}

