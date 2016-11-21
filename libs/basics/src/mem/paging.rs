
prelude!();

use ::core::ops::{Index, IndexMut};


extern {
    static mut PML4: PagingTable;
    static mut PML3: PagingTable;
}

pub const MEMORY_START: usize = (!(1 << 47)) + 1 /* = 2^64 - 2^47*/;
pub const KERNEL_START: usize = 0xffffffff80000000;

pub const PAGE_SIZE_POWER: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_POWER;
pub const TABLE_SIZE: usize = 512;
const GB: usize = 1 << 30;


// Entry:

pub const LAST_LEVEL: usize = 4;

const PRESENT: usize =         1 << 0;
const WRITABLE: usize =        1 << 1;
const USER_ACCESSIBLE: usize = 1 << 2;
#[allow(dead_code)]
const ACCESSED: usize =        1 << 5;
const HUGE_PAGE: usize =       1 << 7;
#[allow(dead_code)]
const NO_EXECUTE: usize =      1 << 63;

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Default)]
pub struct Entry {
    bits: usize,
}

/*pub enum Next<'a> {
    Table(&'a PagingTable),
    Page(*const u8),
    NotPresent,
}

pub enum NextMut<'a> {
    Table(&'a mut PagingTable),
    Page(*mut u8),
    NotPresent,
    NotWritable,
}*/

impl Entry {
    pub const fn new_empty() -> Self {
        Entry { bits: 0 }
    }

    /// Insert bit flags
    pub fn insert(&mut self, flags: usize) {
        self.bits |= flags;
    }

    /// Remove bit flags
    pub fn remove(&mut self, flags: usize) {
        self.bits &= !flags;
    }

    pub fn set_page_phys(&mut self, start: usize) {
        self.clear_and_set_phys_address(start);
        self.insert(PRESENT | HUGE_PAGE | WRITABLE | USER_ACCESSIBLE);
        // self.remove(ACCESSED | NO_EXECUTE);
    }

    /*///
    pub fn go_next(&self, level: usize) -> Next {
        if !self.contains(PRESENT) {
            Next::Fail
        } else if level == LAST_LEVEL || self.contains(HUGE_PAGE) {
            Next::Page(self.get_address())
        } else {
            Next::Table(unsafe { &*(self.get_address() as *const Gdt) })
        }
    }

    pub fn go_next_mut(&self, level: usize) -> NextMut {
        if !self.contains(PRESENT | WRITABLE) {
            NextMut::Fail
        } else if level == LAST_LEVEL || self.contains(HUGE_PAGE) {
            NextMut::Page(self.get_address() as *mut u8)
        } else {
            NextMut::Table(unsafe { &mut *(self.get_address() as *mut Gdt) })
        }
    }*/

    unsafe fn set_next_level_phys(&mut self, phys_addr: usize) {
        self.clear_and_set_phys_address(phys_addr);
        self.insert(PRESENT | WRITABLE | USER_ACCESSIBLE);
        // self.remove(HUGE_PAGE | ACCESSED | NO_EXECUTE);
    }

    fn clear_and_set_phys_address(&mut self, phys_addr: usize) {
        assert_eq!(0, phys_addr & ((1 << 12) - 1));  // address is divisible by 4096
        assert!(phys_addr < (1 << 48));  // address is not too big
        self.bits = phys_addr;
    }
}


// PagingTable

#[repr(C)]
#[repr(packed)]
pub struct PagingTable {
    entries: [Entry; TABLE_SIZE],
}

impl PagingTable {
    pub const fn new_empty() -> Self {
        PagingTable { entries: [Entry::new_empty(); TABLE_SIZE] }
    }
}

impl Index<usize> for PagingTable {
    type Output = Entry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PagingTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

static mut INITIALIZED: bool = false;

pub unsafe fn init_default() {
    if !INITIALIZED {
        INITIALIZED = true;

        let pml3_phys = &PML3 as *const _ as usize - KERNEL_START;
        // identical mapping. [0..] to [0..]
        PML4[0].set_next_level_phys(pml3_phys);
        // [-2^47..] to [0..]
        PML4[TABLE_SIZE / 2].set_next_level_phys(pml3_phys);
        // for kernel section
        PML4[TABLE_SIZE - 1].set_next_level_phys(pml3_phys);

        // identical
        for i in 0..(TABLE_SIZE - 2) {
            PML3[i].set_page_phys(i * GB);
        }

        // Kernel Section: last 2 GB of logical memory to the first 2 GB of physical memory
        PML3[TABLE_SIZE - 2].set_page_phys(0);
        PML3[TABLE_SIZE - 1].set_page_phys(1 * GB);
    }
}

#[cfg(os_test)]
pub mod paging_tests {
    tests_module!("paging",
        todo,
    );

    fn todo() {
        // TODO
    }
}
