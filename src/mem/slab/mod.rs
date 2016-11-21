
use ::prelude::*;
use mem::inplace_list::{ self, InplaceList };
use mem::buddy::*;
use mem::paging::PAGE_SIZE;
use core::marker::PhantomData;
use core::cmp::Eq;

type Node = inplace_list::Node<()>;
type PageNode<'a> = inplace_list::Node<Page<'a>>;

/// Meta information about allocated page.
/// Lifetime parameter is equal to its slab allocator lifetime.
struct Page<'a> {
    /// Ownership over the memory.
    bbox: BuddyBox,
    /// The number of allocated frames on this page.
    cnt: usize,
    /// A slab allocator, which owns this page.
    allocator: Shared<SlabAllocator<'a>>,
    phantom: PhantomData<&'a u8>
}

impl<'a> Page<'a> {
    /// Allocates one page in BuddyAllocator.
    fn allocate(allocator: &'a mut SlabAllocator) -> Option<Page<'a>> {
        Some(Page {
            bbox: tryo!(unsafe { BuddyAllocator::get_instance().allocate_level(0) }),
            cnt: 0,
            allocator: unsafe { Shared::new(allocator) },
            phantom: PhantomData,
        })
    }

    /// The number of allocated frames on this page.
    fn allocated(&self) -> usize {
        self.cnt
    }

    /// Increase the counter of allocated frames on this page.
    fn inc(&mut self) {
        self.cnt += 1;
    }

    /// Decrease the counter of allocated frames on this page.
    fn dec(&mut self) {
        debug_assert!(self.cnt != 0);
        self.cnt -= 1;
    }

    /// Returns a pointer to the start of the page.
    fn ptr(&self) -> *mut u8 {
        **self.bbox
    }

    /// Returns a reference to a slab allocator, which owns this page.
    unsafe fn get_allocator(&mut self) -> &'a mut SlabAllocator {
        &mut **self.allocator
    }
}

// https://github.com/rust-lang/rfcs/issues/1144

/// Must be at least size_of::<Node>()
pub const MIN_FRAME_SIZE: usize = 16;
/// 2 * MAX_FRAME_SIZE should be not more than amount of free space in page
pub const MAX_FRAME_SIZE: usize = PAGE_SIZE / 2 - 16;

/// In order to minimize BuddyAllocator calls, it never frees its last allocated page.
/// So, after the first allocation, there is always at least one page in allocator.
pub struct SlabAllocator<'a> {
    frame_size: usize,
    frames: InplaceList<()>,
    // the first page may be not completely initialized
    pages: InplaceList<Page<'a>>,
    /// The number of nodes initialized in the first page.
    /// Provides lazy page initialization. It's extremely beneficial in some cases.
    last_page_initialized: usize,
}


impl<'a> SlabAllocator<'a> {
    /// `MIN_FRAME_SIZE` ≤ `frame_size` ≤ `MAX_FRAME_SIZE` .
    /// `frame_size` must be divisible by 8 for correct alignment.
    /// The function has to be unchecked in order to be `const`
    pub const unsafe fn new_unchecked(frame_size: usize) -> Self {
        SlabAllocator {
            frame_size: frame_size,
            frames: InplaceList::new(),
            pages: InplaceList::new(),
            last_page_initialized: usize::max_value(),
        }
    }

    /// Same as `new_unchecked`, but checks if `frame_size` is incorrect.
    pub fn new(frame_size: usize) -> Self {
        Self::check_size(frame_size);

        unsafe { Self::new_unchecked(frame_size) }
    }

    pub fn allocate(&mut self) -> Option<NonZero<*mut u8>> {
        // FIXME: avoid reborrow_mut
        let self2 = unsafe { reborrow_mut!(self, Self) };

        if let Some(node) = self.frames.last_mut() {
            unsafe { self2.frames.remove(node); }

            let res = node as *mut Node as *mut u8;

            unsafe { Self::get_page_node(res).as_mut().inc() };
            debug_assert!(
                unsafe { Self::get_page_node(res).as_ref().allocated() <= self.frames_per_page() }
            );

            debug_assert!(res as usize != 0);
            return Some(unsafe { NonZero::new(res) });
        }

        if self.last_page_initialized >= self.frames_per_page() {
            // we have to allocate new page
            tryo!(self.allocate_page());
        }

        self.initialize_node();
        self.allocate()  // recursion
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        debug_assert!((ptr as usize) != 0);
        let page_node: &mut PageNode<'a> = Self::get_page_node(ptr);
        page_node.as_mut().dec();
        self.add_node(ptr);

        // if the page is empty and it's not the last allocated page
        if page_node.as_ref().allocated() == 0 && *page_node != *self.pages.last().unwrap() {
            // deallocate the page
            let page_start = page_node.as_ref().ptr();
            let mut ptr = unsafe { page_start.offset(size_of::<PageNode>() as isize) };
            while (ptr as usize) + self.frame_size <= (page_start as usize) + PAGE_SIZE {
                unsafe {
                    self.frames.remove(&mut *(ptr as *mut Node));
                    ptr = ptr.offset(self.frame_size as isize);
                }
            }
        }
    }

    /// Deallocates memory without a reference to its allocator
    pub unsafe fn deallocate_unknown(ptr: *mut u8) {
        let page_node: &mut PageNode<'a> = Self::get_page_node(ptr);
        page_node.as_mut().get_allocator().deallocate(ptr);
    }

    /// Necessary to check size of slab allocator which was created by call of `new_unchecked`
    pub fn check_correctness(&self) {
        Self::check_size(self.frame_size)
    }

    fn check_size(frame_size: usize) {
        assert!(frame_size <= MAX_FRAME_SIZE);
        assert!(frame_size >= MIN_FRAME_SIZE);
        assert!(frame_size % 8 == 0);
    }

    fn allocate_page(&mut self) -> Option<()> {
        let page: Page<'a> = unsafe {
            // FIXME: avoid reborrow_mut
            let self2 = reborrow_mut!(self);
            tryo!(Page::allocate(self2))
        };

        let page_start = page.ptr();

        unsafe {
            ptr::write(page_start as *mut PageNode, PageNode::new(page));
            self.pages.insert(&mut *(page_start as *mut PageNode));
        };

        self.last_page_initialized = 0;

        debug_assert!(self.pages.last().is_some());
        Some(())
    }

    fn initialize_node(&mut self) {
        let page_begin = self.pages.last_mut().unwrap().as_mut().ptr();
        let offset = size_of::<PageNode>() + self.frame_size * self.last_page_initialized;
        debug_assert!(offset + self.frame_size <= PAGE_SIZE);
        unsafe {
            let ptr = page_begin.offset(offset as isize);
            self.add_node(ptr);
        }
        self.last_page_initialized += 1;

        debug_assert!(self.frames.last_mut().is_some());

    }

    unsafe fn add_node(&mut self, ptr: *mut u8) {
        let p = ptr as *mut Node;
        ptr::write(p, Node::new(()));
        self.frames.insert(&mut *(p));
    }

    unsafe fn get_page_node(ptr: *mut u8) -> &'a mut PageNode<'a> {
        // page node is in the begin of a page
        &mut *(utility::round_down(ptr as usize, PAGE_SIZE) as *mut PageNode)
    }

    fn frames_per_page(&self) -> usize {
        (PAGE_SIZE - size_of::<PageNode>()) / self.frame_size
    }
}


#[cfg(os_test)]
pub mod slab_tests {
    use super::*;
    use super::{ Node, PageNode };
    use mem::paging::PAGE_SIZE;

    tests_module!("slab_allocator",
        min_frame_at_least_node_size,
        simple,
    );

    /// It is necessary to fit Node in not-allocated frames.
    fn min_frame_at_least_node_size() {
        assert!(MIN_FRAME_SIZE >= size_of::<Node>());
    }

    fn simple() {
        const N: usize = 200;
        const SIZE: usize = 160;

        let mut allocator = SlabAllocator::new(SIZE);
        let mut ptrs = [None; N];

        let mut allocate = || {
            for i in 0..N {
                ptrs[i] = allocator.allocate();
                // allocated objects don't intersect
                for j in 0..i {
                    let addr1 = *ptrs[j].unwrap() as isize;
                    let addr2 = *ptrs[i].unwrap() as isize;
                    assert!((addr1 - addr2).abs() >= SIZE as isize);
                }
            }

            for i in 0..N {
                unsafe { allocator.deallocate(*ptrs[i].unwrap()) };
                ptrs[i] = None;
            }
        };

        allocate();
        allocate();
    }
}

