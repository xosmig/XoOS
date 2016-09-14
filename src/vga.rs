
const COLOR_BYTE: u8 = 0b0_001_1111; // white foreground, blue background

use super::core::ptr;

const VGA_MEM: u64 = 0xb8000;

pub fn clear() {
    for ptr in (VGA_MEM + 1000)..(VGA_MEM + 3000) {
        unsafe { ptr::write(ptr as *mut _, 0 as u16) };
    }
}

pub fn print(str: &[u8]) {
    let offset = 2004 - str.len() + if str.len() % 2 == 1 {1} else {0}; // it must be even
    let vga_mem = 0xb8000 + offset;

    clear();
    for (i, char) in str.into_iter().enumerate() {
        unsafe {
            ptr::write((vga_mem + i * 2) as *mut _, *char);
            ptr::write((vga_mem + i * 2 + 1) as *mut _, COLOR_BYTE);
        }
    }
}
