use ::core::mem::{size_of, size_of_val};
use ::fmt::{self, Formatter, Debug};

#[allow(improper_ctypes)]
extern {
    static TEXT_PHYS_BEGIN: ();
    static BSS_PHYS_END: ();
}

/// Pointer to a memory map given from multiboot
#[repr(C)]
#[repr(packed)]
#[derive(Debug)]
pub struct MemoryMapPtr {
    length: u32,
    addr: u32,
}

impl MemoryMapPtr {
    pub fn iter(&self) -> Iter {
        Iter {
            entry: unsafe { &*(self.addr as *const _) },
            end: (self.addr + self.length) as *const _,
        }
    }
}

pub const MMAP_MAX_LEN: usize = 32;
pub const ERROR_MSG: &'static str = "FAIL! Memory map is too big.";

#[derive(Default)]
pub struct MemoryMap {
    entries: [Entry; MMAP_MAX_LEN],
    len: usize,
}

impl MemoryMap {
    pub fn load(ptr: &MemoryMapPtr) -> Self {
        let kernel_start = unsafe { &TEXT_PHYS_BEGIN as *const () as *const u8 };
        let kernel_end = unsafe { &BSS_PHYS_END as *const () as *const u8 };

        let mut ret = MemoryMap::default();

        // I can't use zip and have to use it to insert the kernel section to the memory map.
        let mut len = 0;
        for entry in ptr.iter() {
            // - 2 because we need place to insert the kernel section
            assert!(ret.len < MMAP_MAX_LEN - 2, ERROR_MSG);

            if entry.start() < kernel_start && entry.end() >= kernel_end {
                // insert the kernel section
                assert!(entry.typ == EntryType::Available);

                ret.entries[len] = Entry::new(entry.start(), kernel_start, EntryType::Available);
                ret.entries[len + 1] = Entry::new(kernel_start, kernel_end, EntryType::Occupied);
                ret.entries[len + 2] = Entry::new(kernel_end, entry.end(), EntryType::Available);
                len += 2;
            } else {
                ret.entries[len] = *entry;
                ret.entries[len].skip_to_next = (size_of::<Entry>() - size_of::<u32>()) as u32;
            }

            len += 1;
        }
        ret.len = len;

        ret
    }

    pub fn iter(&self) -> Iter {
        Iter {
            entry: &self.entries[0],
            end: &self.entries[self.len],
        }
    }
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, entry) in self.iter().enumerate() {
            try!(writeln!(f, "{}: [{:?}; {:?}), len: {} bytes, type: {:?}",
                          i,
                          entry.start(),
                          entry.end(),
                          entry.len(),
                          entry.typ
                )
            );
        }
        Ok(())
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntryType {
    Available = 1,
    Occupied = 2,
}

impl Default for EntryType {
    fn default() -> Self {
        EntryType::Occupied
    }
}

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Entry {
    skip_to_next: u32,
    start: u64,
    len: u64,
    typ: EntryType,
}

impl Entry {
    fn new(start: *const u8, end: *const u8, typ: EntryType) -> Entry {
        Entry {
            skip_to_next: (size_of::<Entry>() - size_of::<u32>()) as u32,
            start: start as usize as u64,
            len: (end as usize - start as usize) as u64,
            typ: typ,
        }
    }

    pub fn start(&self) -> *const u8 {
        self.start as *const u8
    }

    pub fn end(&self) -> *const u8 {
        unsafe { self.start().offset(self.len as isize) }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_available(&self) -> bool {
        self.typ == EntryType::Available
    }

    unsafe fn get_next(&self) -> &'static Self {
        &*((self as *const _ as *const u8).offset(
            (self.skip_to_next() + size_of_val(&self.skip_to_next)) as isize
        ) as *const Entry)
    }

    fn skip_to_next(&self) -> usize {
        return self.skip_to_next as usize;
    }
}

pub struct Iter<'a> {
    entry: &'a Entry,
    end: *const Entry,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<&'a Entry> {
        if self.entry as *const _ < self.end {
            let ret = self.entry;
            self.entry = unsafe { self.entry.get_next() };
            Some(ret)
        } else {
            None
        }
    }
}
