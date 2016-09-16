
use ::vga;
use ::core::fmt;

// FIXME: Should be the entry point on panic.
#[allow(private_no_mangle_fns)]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    vga::print(b"ERROR: panic!");
    loop {}
}

mod unwinding_fix {
    // Unwinding is disabled in Cargo.toml, so this functions will never be called (probably).

    use ::vga;

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
