
mod single;

use ::core::ptr::Shared;
use ::mem::memory_map::{ MMAP_MAX_LEN as MAX_FRAMES_CNT, MemoryMap };
use self::single::Single;

struct BuddyAllocator {
    singles: [Option<Shared<Single>>; MAX_FRAMES_CNT],
}

// horrible hack
static mut INSTANCE: BuddyAllocator = BuddyAllocator { singles: [None; MAX_FRAMES_CNT] };
static mut INITIALIZED: bool = false;

pub unsafe fn init_default(mmap: &MemoryMap) {
    if !INITIALIZED {
        INITIALIZED = true;

        let mut cnt = 0;
        for entry in mmap.iter() {
            if let Some(single_ref) = Single::new(entry) {
                INSTANCE.singles[cnt] = Some(Shared::new(single_ref));
                cnt += 1;
            }
        }
    }
}
