
// The entry point on panic.
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! {
    loop {} // FIXME
}

mod unwinding_fix {
    // Unwinding is disabled in Cargo.toml, so this functions will never be called (probably).

    #[lang = "eh_personality"]
    pub extern fn eh_personality() {
        loop {} // FIXME
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn _Unwind_Resume() -> ! {
        loop {} // FIXME
    }
}
