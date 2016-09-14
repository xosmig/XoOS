
use super::vga;

// The entry point on panic.
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! {
    vga::print(b"ERROR: panic_fmt was executed.");
    loop {}
}

mod unwinding_fix {
    // Unwinding is disabled in Cargo.toml, so this functions will never be called (probably).

    use super::super::vga;

    #[lang = "eh_personality"]
    pub extern fn eh_personality() {
        vga::print(b"ERROR: eh_personality was executed.");
        loop {}
    }

    #[allow(private_no_mangle_fns)]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn _Unwind_Resume() -> ! {
        vga::print(b"ERROR: _Unwind_Resume was executed.");
        loop {}
    }
}
