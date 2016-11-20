
use ::prelude::*;
use mem::inplace_list::{ self, InplaceList };
use mem::buddy::*;
use mem::paging::PAGE_SIZE;
use core::marker::PhantomData;

type Node = inplace_list::Node<()>;
type PageNode<'a> = inplace_list::Node<Page<'a>>;

struct Page<'a> {
    bbox: BuddyBox,
    /// The number of allocated frames on this page.
    cnt: usize,
    allocator: Shared<SlabAllocator<'a>>,
    phantom: PhantomData<&'a u8>
}

impl<'a> Page<'a> {
    fn allocate(allocator: &'a mut SlabAllocator) -> Option<Page<'a>> {
        Some(Page {
            bbox: tryo!(unsafe { BuddyAllocator::get_instance().allocate_level(0) }),
            cnt: 0,
            allocator: unsafe { Shared::new(allocator) },
            phantom: PhantomData,
        })
    }

    fn allocated(&self) -> usize {
        self.cnt
    }

    fn inc(&mut self) {
        self.cnt += 1;
    }

    fn dec(&mut self) {
        debug_assert!(self.cnt != 0);
        self.cnt -= 1;
    }

    fn ptr(&self) -> NonZero<*mut u8> {
        *self.bbox
    }

    unsafe fn get_allocator(&mut self) -> &'a mut SlabAllocator {
        &mut **self.allocator
    }
}

// https://github.com/rust-lang/rfcs/issues/1144

/// Must be at least size_of::<Node>()
pub const MIN_FRAME_SIZE: usize = 16;
/// 2 * MAX_FRAME_SIZE should be not more than amount of free space in page
pub const MAX_FRAME_SIZE: usize = PAGE_SIZE / 2 - 16;

pub struct SlabAllocator<'a> {
    frame_size: usize,
    frames: InplaceList<()>,
    // the first page may be not completely initialized
    pages: InplaceList<Page<'a>>,
    /// The number of nodes initialized in the first page.
    /// Provides lazy page initialization. It's extremely beneficial in some cases.
    first_page_initialized: usize,
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
            first_page_initialized: usize::max_value(),
        }
    }

    /// Same as `new_unchecked`, but checks if `frame_size` is incorrect.
    pub fn new(frame_size: usize) -> Self {
        assert!(frame_size <= MAX_FRAME_SIZE);
        assert!(frame_size >= MIN_FRAME_SIZE);
        assert!(frame_size % 8 == 0);

        unsafe { Self::new_unchecked(frame_size) }
    }

    pub fn allocate(&mut self) -> Option<NonZero<*mut u8>> {
        // FIXME: avoid reborrow_mut
        let self2 = unsafe { reborrow_mut!(self, Self) };

        if let Some(node) = self.frames.first_mut() {
            unsafe { self2.frames.remove(node); }

            let res = node as *mut Node as *mut u8;

            unsafe { Self::get_page_node(res).as_mut().inc() };
            debug_assert!(
                unsafe { Self::get_page_node(res).as_ref().allocated() <= self.frames_per_page() }
            );

            debug_assert!(res as usize != 0);
            return Some(unsafe { NonZero::new(res) });
        }

        if self.first_page_initialized >= self.frames_per_page() {
            // we have to allocate new page
            tryo!(self.allocate_page());
        }

        self.initialize_node();
        self.allocate()  // recursion
    }

    /*pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        debug_assert!(ptr != 0);
        let page_node = Self::get_page_node(ptr);
        page_node.as_mut().dec();
//        if page_node.as_ref().allocated() == 0 {
//            deallocate_page;
//        }
        self.add_node(ptr);
    }*/

    fn allocate_page(&mut self) -> Option<()> {
        let page: Page<'a> = unsafe {
            // FIXME: avoid reborrow_mut
            let self2 = reborrow_mut!(self);
            tryo!(Page::allocate(self2))
        };

        let page_start = *page.ptr();

        unsafe {
            ptr::write(page_start as *mut PageNode, PageNode::new(page));
            self.pages.insert(&mut *(page_start as *mut PageNode));
        };

        self.first_page_initialized = 0;

        debug_assert!(self.pages.first().is_some());
        Some(())
    }

    fn initialize_node(&mut self) {
        let page_begin = *self.pages.first_mut().unwrap().as_mut().ptr();
        let offset = size_of::<PageNode>() + self.frame_size * self.first_page_initialized;
        debug_assert!(offset + self.frame_size <= PAGE_SIZE);
        unsafe {
            let ptr = page_begin.offset(offset as isize);
            self.add_node(ptr);
        }
        self.first_page_initialized += 1;

        debug_assert!(self.frames.first_mut().is_some());

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
        simple_allocate_test,
    );

    // it is necessary for correct alignment
    fn min_frame_at_least_node_size() {
        assert!(MIN_FRAME_SIZE >= size_of::<Node>());
    }

    fn simple_allocate_test() {
        const N: usize = 100;
        const SIZE: usize = 160;

        let mut allocator = SlabAllocator::new(SIZE);
        let mut ptrs = [None; N];

        for i in 0..N {
            ptrs[i] = allocator.allocate();
            // allocated objects don't intersect
            for j in 0..i {
                let addr1 = *ptrs[j].unwrap() as isize;
                let addr2 = *ptrs[i].unwrap() as isize;
                assert!((addr1 - addr2).abs() >= SIZE as isize);
            }
        }
    }
}

