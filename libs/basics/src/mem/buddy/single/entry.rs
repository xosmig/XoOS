
prelude!();

pub struct Entry {
    num: u32,
    level: u16,
    occupied: bool,
}

impl Entry {
    pub const fn new(num: usize) -> Self {
        Entry { num: num as u32, level: 0, occupied: false }
    }

    pub fn num(&self) -> usize {
        self.num as usize
    }

    pub fn level(&self) -> usize {
        self.level as usize
    }

    pub fn set_free(&mut self) {
        self.occupied = false;
    }

    pub fn set_occupied(&mut self) {
        self.occupied = true;
    }

    pub fn get_buddy(&self) -> usize {
        self.buddy_on_level(self.level as usize)
    }

    pub fn buddy_on_level(&self, level: usize) -> usize {
        self.num() ^ (1 << level)
    }

    pub fn is_free(&self) -> bool {
        !self.is_occupied()
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level as u16;
    }

    pub fn ready(&self, buddy: &Entry) -> bool {
        let ret = self.is_free() && self.level() == buddy.level();
        if ret {
            debug_assert!(buddy.num() == self.get_buddy());
        }
        ret
    }
}
