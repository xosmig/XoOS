
mod single;

use ::prelude::*;
use ::core::ops::Deref;
use ::mem::paging::PAGE_SIZE;
use ::utility::log2_ceil;
use ::mem::memory_map::{ MMAP_MAX_LEN as MAX_FRAMES_CNT, MemoryMap };
use self::single::Single;


#[derive(PartialEq, Eq)]
pub struct BuddyBox {
    single_num: usize,
    pointer: NonZero<*mut u8>,
}

/// Similar to std::boxed::Box.
/// Provides ownership for an allocation, and drop its content when it go out of scope.
impl Deref for BuddyBox {
    type Target = *mut u8;
    fn deref(&self) -> &Self::Target {
        &(*self.pointer)
    }
}

impl Drop for BuddyBox {
    fn drop(&mut self) {
        unsafe { BuddyAllocator::get_instance().deallocate(self) };
    }
}


pub struct BuddyAllocator {
    singles: [Option<Shared<Single>>; MAX_FRAMES_CNT],
}

static mut INSTANCE: BuddyAllocator = BuddyAllocator { singles: [None; MAX_FRAMES_CNT] };
static mut INITIALIZED: bool = false;

impl BuddyAllocator {

    /// unsafe because it depends on mmap correctness
    pub unsafe fn init_default(mmap: &MemoryMap) {
        if !INITIALIZED {
            let mut cnt = 0;
            for entry in mmap.iter() {
                if let Some(single_ref) = Single::new(entry) {
                    INSTANCE.singles[cnt] = Some(Shared::new(single_ref));
                    cnt += 1;
                }
            }
            INITIALIZED = true;
        }
    }

    pub fn get_instance() -> &'static mut Self {
        unsafe { &mut INSTANCE }
    }

    pub fn allocate_level(&mut self, level: usize) -> Option<BuddyBox> {
        let mut num = 0;
        while let Some(ptr) = self.singles[num] {
            if let Some(address) = unsafe{ (**ptr).allocate(level) } {
                return Some(BuddyBox { single_num: num, pointer: address });
            }
            num += 1;
        }

        None
    }

    pub fn allocate(&mut self, size: usize) -> Option<BuddyBox> {
        self.allocate_level(Self::size_to_level(size))
    }

    pub fn size_to_level(size: usize) -> usize {
        debug_assert!(size > 0);
        log2_ceil((size + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    unsafe fn deallocate(&mut self, bbox: &BuddyBox) {
        (**(self.singles[bbox.single_num].unwrap())).deallocate(bbox.pointer);
    }
}

#[cfg(os_test)]
pub mod buddy_tests {
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
        let page1 = allocator.allocate(123).unwrap();
        let page2 = allocator.allocate(123).unwrap();
        let page3 = allocator.allocate(4096 * 2).unwrap();
        let page4 = allocator.allocate(4096 * 10).unwrap();

        assert!(*page1 as usize % 4096 == 0);
        assert!(*page2 as usize % 4096 == 0);
        assert!(*page3 as usize % 4096 == 0);
        assert!(*page4 as usize % 4096 == 0);

        assert!(page1 != page2);
        assert!(page1 != page3);
        assert!(page1 != page4);
        assert!(page2 != page3);
        assert!(page2 != page4);
        assert!(page3 != page4);
    }

    fn allocate_big_twice_test() {
        const N: usize = 32;

        let allocate_max = || generate![max_possible_page(); N];

        // assert allocate_max correctness
        {
            let _foo = allocate_max();
            let bar = allocate_max();  // must be empty
            for x in bar.iter() {
                assert!(x.is_none());
            }
        }

        let _page = BuddyAllocator::get_instance().allocate_level(0);
        let allocate_max_levels = || {
            let pages = allocate_max();
            let mut levels = [0; N];
            for (i, item) in pages.iter().enumerate() {
                levels[i] = match item.as_ref() {
                    Some(pair) => pair.1 as isize,
                    None => -1,
                };
            }
            levels
        };

        // Allocate all, clear, allocate all again.
        // Results of allocations must be identical.
        {
            let levels1 = allocate_max_levels();
            let levels2 = allocate_max_levels();
            assert_eq!(levels1, levels2);
        }

        // Allocate all, clear, allocate some memory, allocate all again.
        // Results of allocations must be different.
        {
            let levels1 = allocate_max_levels();
            let _page = BuddyAllocator::get_instance().allocate_level(0);
            let levels2 = allocate_max_levels();
            assert_ne!(levels1, levels2);
        }

    }

    fn max_possible_page() -> Option<(BuddyBox, usize)> {
        let allocator = BuddyAllocator::get_instance();
        for level in (0..32).rev() {
            let res = allocator.allocate_level(level);
            if let Some(page) = res {
                return Some((page, level));
            }
        }
        None
    }

    pub fn all() {
        size_to_level_test();
        allocate_test();
        allocate_big_twice_test();
    }
}
