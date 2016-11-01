
mod single;

use ::core::*;  // FIXME: use own prelude instead of core
use ::mem::paging::PAGE_SIZE;
use ::utility::log2_ceil;
use ::core::ptr::Shared;
use ::core::nonzero::NonZero;
use ::mem::memory_map::{ MMAP_MAX_LEN as MAX_FRAMES_CNT, MemoryMap };
use self::single::Single;

pub struct BuddyAllocator {
    singles: &'static mut [&'static mut Single],
}

static mut INSTANCE: BuddyAllocator = BuddyAllocator { singles: &mut [] };
static mut MEM_FOR_SINGLES: Option<[&'static mut Single; MAX_FRAMES_CNT]> = None;
static mut INITIALIZED: bool = false;

impl BuddyAllocator {

    /// unsafe because it depends on mmap correctness
    pub unsafe fn init_default(mmap: &MemoryMap) {
        if !INITIALIZED {
            INITIALIZED = true;
            MEM_FOR_SINGLES = Some(mem::uninitialized());
            let singles = MEM_FOR_SINGLES.as_mut().unwrap();

            let mut cnt = 0;
            for entry in mmap.iter() {
                if let Some(single_ref) = Single::new(entry) {
                    ptr::write(
                        &mut singles[cnt],
                        single_ref,
                    );
                    cnt += 1;
                }
            }

            INSTANCE.singles = &mut singles[..cnt];
        }
    }

    pub fn get_instance() -> &'static mut Self {
        unsafe { &mut INSTANCE }
    }

    pub fn allocate_level(&mut self, level: usize) -> Option<NonZero<*mut u8>> {
        for single in self.singles.iter_mut() {
            if let Some(ret) = single.allocate(level) {
                return Some(ret);
            }
        }

        None
    }

    pub fn allocate(&mut self, size: usize) -> Option<NonZero<*mut u8>> {
        self.allocate_level(Self::size_to_level(size))
    }

    pub fn size_to_level(size: usize) -> usize {
        debug_assert!(size > 0);
        log2_ceil((size + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        // TODO
    }
}

#[cfg(os_test)]
pub mod tests {
    use super::*;

    fn size_to_level_test() {
        assert_eq!(0, BuddyAllocator::size_to_level(1));
        assert_eq!(0, BuddyAllocator::size_to_level(4000));
        assert_eq!(0, BuddyAllocator::size_to_level(4096));
        assert_eq!(1, BuddyAllocator::size_to_level(4097));
        assert_eq!(1, BuddyAllocator::size_to_level(4096 * 2));
        assert_eq!(2, BuddyAllocator::size_to_level(4096 * 2 + 1));
        assert_eq!(5, BuddyAllocator::size_to_level(4096 * (1 << 5)));
        assert_eq!(6, BuddyAllocator::size_to_level(4096 * (1 << 6) - 1));
    }

    fn allocate_test() {
        let allocator = BuddyAllocator::get_instance();
        let page1 = *(allocator.allocate(123).unwrap());
        let page2 = *(allocator.allocate(123).unwrap());
        let page3 = *(allocator.allocate(4096 * 2).unwrap());
        let page4 = *(allocator.allocate(4096 * 10).unwrap());
        debug_assert!(page1 != page2);
        debug_assert!(page1 != page3);
        debug_assert!(page1 != page4);
        debug_assert!(page2 != page3);
        debug_assert!(page2 != page4);
        debug_assert!(page3 != page4);
    }

    pub fn all() {
        size_to_level_test();
        allocate_test();
    }
}

