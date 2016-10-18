
use ::core::mem::size_of_val;
use ::fmt::{Debug, Formatter, Error};

#[repr(C)]
#[repr(packed)]
pub struct MemoryMap {
    length: u32,
    addr: u32,
}

impl MemoryMap {
    pub fn first_entry(&self) -> &'static Entry {
        unsafe { &*(self.addr as *const Entry) }
    }

    pub fn end(&self) -> *const Entry {
        (self.addr + self.length) as *const _
    }

    pub fn iter(&self) -> Iter {
        Iter {
            entry: self.first_entry(),
            end: self.end(),
        }
    }
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (i, entry) in self.iter().enumerate() {
            try!(writeln!(f, "{}: [{:?}; {:?}), len: {}, type: {}",
                          i,
                          entry.start(),
                          entry.end(),
                          entry.len(),
                          entry.typ as u32
                )
            );
        }
        Ok(())
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum EntryType {
    Available = 1,
    Occupied = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    size: u32,
    start: u64,
    len: u64,
    typ: EntryType,
}

impl Entry {
    pub fn start(&self) -> *const u8 {
        self.start as *const u8
    }

    pub fn end(&self) -> *const u8 {
        unsafe { self.start().offset(self.len as isize) }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    unsafe fn get_next(&self) -> &'static Self {
        &*((self as *const _ as *const u8).offset(
            (self.size() + size_of_val(&self.size)) as isize
        ) as *const Entry)
    }

    fn size(&self) -> usize {
        return self.size as usize;
    }
}

pub struct Iter {
    entry: &'static Entry,
    end: *const Entry,
}

impl Iterator for Iter {
    type Item = &'static Entry;

    fn next(&mut self) -> Option<&'static Entry> {
        if self.entry as *const _ < self.end {
            let ret = self.entry;
            self.entry = unsafe { self.entry.get_next() };
            Some(ret)
        } else {
            None
        }
    }
}
