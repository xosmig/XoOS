
const COLOR_BYTE: u8 = 0x1f; // white foreground, blue background
const MAX_LENGTH: usize = 32;

pub fn print(str: &[u8]) {

    assert!(str.len() < MAX_LENGTH);

    let mut colored = [0; 2 * MAX_LENGTH];
    for (i, char_byte) in str.into_iter().enumerate() {
        colored[i * 2] = *char_byte;
        colored[i * 2 + 1] = COLOR_BYTE;
    }

    // write colored text to the center of the VGA text buffer
    let offset = 2004 - str.len() + if str.len() % 2 == 1 {1} else {0}; // it must be even
    let buffer_ptr = (0xb8000 + offset) as *mut _;
    unsafe { *buffer_ptr = colored };
}
