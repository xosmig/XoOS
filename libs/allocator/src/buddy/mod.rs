
prelude!();

mod single;

use ::core::ops::Deref;
use ::mem::paging::PAGE_SIZE;
use ::utility::log2_ceil;
use ::mem::memory_map::{ MMAP_MAX_LEN as MAX_FRAMES_CNT, MemoryMap };
use self::single::Single;
use ::core::sync::atomic::*;
use ::sync::{ SpinMutexGuard, SpinMutex };


#[derive(PartialEq, Eq)]
pub struct BuddyRaw {
    pub single_num: usize,
    pub pointer: NonZero<*mut u8>,
}

#[derive(PartialEq, Eq)]
pub struct BuddyBox {
    raw: BuddyRaw,
}

impl BuddyBox {
    pub fn get(&self) -> *mut u8 {
        *self.raw.pointer
    }
}

/// Similar to std::boxed::Box.
/// Provides ownership for an allocation, and drop its content when it go out of scope.
impl Deref for BuddyBox {
    type Target = NonZero<*mut u8>;
    fn deref(&self) -> &Self::Target {
        &self.raw.pointer
    }
}

impl Drop for BuddyBox {
    fn drop(&mut self) {
        unsafe { BuddyAllocator::lock().deallocate(self) };
    }
}


pub struct BuddyAllocator {
    // I have to use `Shared` singles are stored in static memory so we can't own them.
    singles: [Option<Shared<Single>>; MAX_FRAMES_CNT],
}
// `Shared` doesn't implement Send, but in fact, we own singles.
// So, this `impl` is safe.
unsafe impl Send for BuddyAllocator {}


lazy_static! {
    static ref INSTANCE: SpinMutex<BuddyAllocator> = SpinMutex::new(
        unsafe { BuddyAllocator::uninitialized() }
    );
}
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub type BuddyAllocatorGuard = SpinMutexGuard<'static, BuddyAllocator>;


impl BuddyAllocator {
    const unsafe fn uninitialized() -> Self {
        BuddyAllocator { singles: [None; MAX_FRAMES_CNT] }
    }

    /// Must be called only once in initialization process before threads initialization
    pub unsafe fn init_default(mmap: &MemoryMap) {
        if INITIALIZED.swap(true, Ordering::Relaxed) {
            panic!("BuddyAllocator::init must be called only once");
        }

        let mut cnt = 0;
        for entry in mmap.iter() {
            if let Some(single_ref) = Single::new(entry) {
                // FIXME: thread safety
                unsafe { INSTANCE.lock().singles[cnt] = Some(Shared::new(single_ref)) };
                cnt += 1;
            }
        }
    }

    pub fn lock() -> BuddyAllocatorGuard {
        assert!(INITIALIZED.load(Ordering::Relaxed));
        INSTANCE.lock()
    }

    pub fn allocate_level(&mut self, level: usize) -> Option<BuddyBox> {
        unsafe { self.allocate_level_raw(level).map(|x| BuddyBox { raw: x } ) }
    }

    pub fn allocate(&mut self, size: usize) -> Option<BuddyBox> {
        unsafe { self.allocate_raw(size).map(|x| BuddyBox { raw: x } ) }
    }

    pub unsafe fn allocate_level_raw(&mut self, level: usize) -> Option<BuddyRaw> {
        let mut num = 0;
        while let Some(ptr) = self.singles[num] {
            if let Some(address) = (**ptr).allocate(level) {
                return Some(BuddyRaw { pointer: address, single_num: num });
            }
            num += 1;
        }

        None
    }

    pub unsafe fn deallocate_unknown(&mut self, ptr: *mut u8) {
        assert!(ptr as usize != 0);
        for mb_single in self.singles.iter_mut() {
            if let Some(shared_single) = *mb_single {
                if (**shared_single).contains_addr(ptr) {
                    (**shared_single).deallocate(NonZero::new(ptr));
                }
            } else {
                break;
            }
        }
    }

    pub unsafe fn allocate_raw(&mut self, size: usize) -> Option<BuddyRaw> {
        self.allocate_level_raw(Self::size_to_level(size))
    }

    pub unsafe fn deallocate_raw(&mut self, raw: &BuddyRaw) {
        let single = &mut **(self.singles[raw.single_num].unwrap());
        single.deallocate(raw.pointer);
    }

    fn size_to_level(size: usize) -> usize {
        debug_assert!(size > 0);
        log2_ceil((size + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    unsafe fn deallocate(&mut self, bbox: &BuddyBox) {
        self.deallocate_raw(&bbox.raw);
    }
}


#[cfg(os_test)]
pub mod buddy_tests {
    use super::*;
    tests_module!("buddy_allocator",
        size_to_level_test,
        allocate_test,
        allocate_big_twice_test,
    );

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

    fn allocate_test() {;
        let page1 = BuddyAllocator::lock().allocate(123).unwrap();
        let page2 = BuddyAllocator::lock().allocate(123).unwrap();
        let page3 = BuddyAllocator::lock().allocate(4096 * 2).unwrap();
        let page4 = BuddyAllocator::lock().allocate(4096 * 10).unwrap();

        assert!(**page1 as usize % 4096 == 0);
        assert!(**page2 as usize % 4096 == 0);
        assert!(**page3 as usize % 4096 == 0);
        assert!(**page4 as usize % 4096 == 0);

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

        let _page = unsafe { BuddyAllocator::lock().allocate_level(0) };
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
            let _page = unsafe { BuddyAllocator::lock().allocate_level(0) };
            let levels2 = allocate_max_levels();
            assert_ne!(levels1, levels2);
        }

    }

    fn max_possible_page() -> Option<(BuddyBox, usize)> {
        let mut allocator = BuddyAllocator::lock();
        for level in (0..32).rev() {
            let res = allocator.allocate_level(level);
            if let Some(page) = res {
                return Some((page, level));
            }
        }
        None
    }
}
