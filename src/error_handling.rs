use ::vga;
use ::core::fmt;
use ::fmt::Write;

/// The entry point on panic.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("!! PANIC: in FILE: `{}`, on LINE: `{}`", file, line);
    println!("MESSAGE: `{}`", msg);
    vga::print(b"!!! PANIC !!!");
    loop {}
}

pub mod unwinding_fix {
    //! Unwinding is disabled in Cargo.toml, so this functions will never be called (probably).

    use ::vga;

    #[lang = "eh_personality"]
    pub extern fn eh_personality() {
        vga::print(b"ERROR: eh_personality was executed.");
        loop {}
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn _Unwind_Resume() -> ! {
        vga::print(b"ERROR: _Unwind_Resume was executed.");
        loop {}
    }
}
