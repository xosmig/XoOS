// Now I don't need fmod.

use super::vga;

#[no_mangle]
pub extern fn fmod(_: f64, _: f64) {
    vga::print(b"ERROR: fmod is executed");
    loop {  }
}

#[no_mangle]
pub extern fn fmodf(_: f32, _: f32) {
    vga::print(b"ERROR: fmodf is executed");
    loop {  }
}
