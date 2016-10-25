
use ::utility::*;
use ::mem::memory_map::{MemoryMap, MemoryMapPtr};

#[repr(C)]
#[repr(packed)]
#[derive(Debug)]
pub struct MultibootInfo {
    // Multiboot info version number
    flags: u32,                 // 0
    // Available memory from BIOS
    mem_lower: u32,             // 4
    mem_upper: u32,             // 8
    // root partition
    boot_device: u32,           // 12
    // Kernel command line
    cmdline: u32,               // 16
    // Boot-Module list
    mods_count: u32,            // 20
    mods_addr: u32,             // 24
    // something
    syms_1: u32,                // 28
    syms_2: u32,                // 32
    syms_3: u32,                // 36
    syms_4: u32,                // 40
    // Memory Map
    mmap_ptr: MemoryMapPtr,     // [44; 52)
    // Drive Info buffer
    drives_length: u32,         // 52
    drives_addr: u32,           // 56
    // ROM configuration table
    config_table: u32,          // 60
    // Boot Loader Name
    boot_loader_name: u32,      // 64
    // APM table
    apm_tabe: u32,              // 68
    // Video
    vbe_control_info: u32,      // 72
    vbe_mode_info: u32,         // 76
    vbe_mode: u16,              // 80
    vbe_interface_seg: u16,     // 82
    vbe_interface_off: u16,     // 84
    vbe_interface_len: u16,     // 86
}

impl MultibootInfo {
    pub unsafe fn load(ptr: usize) -> &'static MultibootInfo {
        let ret = &*(ptr as *const MultibootInfo);

        // some checks for correctness
        // assert!(get_bit(ret.flags, 0)); // mem_* fields are present
        assert!(get_bit(ret.flags, 6)); // mmap_* fields are present
        // Bits 4 & 5 are mutually exclusive
        assert!(!get_bit(ret.flags, 4) || !get_bit(ret.flags, 5));

        ret
    }

    pub fn memory_map(&'static self) -> MemoryMap {
        MemoryMap::load(&self.mmap_ptr)
    }
}
