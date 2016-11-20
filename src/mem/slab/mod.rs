
use ::prelude::*;
use mem::inplace_list::{ self, InplaceList };
use mem::buddy::*;
use mem::paging::PAGE_SIZE;

type Node = inplace_list::Node<()>;
type PageNode = inplace_list::Node<Page>;

struct Page {
    bbox: BuddyBox,
    /// the number of allocated frames on this page
    cnt: usize,
}

impl Page {
    fn allocate() -> Option<Self> {
        Some(Page {
            bbox: tryo!(unsafe { BuddyAllocator::get_instance().allocate_level(0) }),
            cnt: 0,
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
}

// https://github.com/rust-lang/rfcs/issues/1144
/// Must be at least size_of::<Node>()
pub const MIN_FRAME_SIZE: usize = 16;
pub const MAX_FRAME_SIZE: usize = PAGE_SIZE / 2 - 1;

pub struct SlabAllocator {
    frame_size: usize,
    frames: InplaceList<()>,
    pages: InplaceList<Page>,
}

impl SlabAllocator {
    /// `MIN_FRAME_SIZE` ≤ `frame_size` ≤ `MAX_FRAME_SIZE` .
    /// `frame_size` must be divisible by 8 for correct alignment.
    pub fn new(frame_size: usize) -> Self {
        assert!(frame_size <= MAX_FRAME_SIZE);
        assert!(frame_size >= MIN_FRAME_SIZE);
        assert!(frame_size % 8 == 0);

        let mut res = SlabAllocator {
            frame_size: frame_size,
            frames: InplaceList::new(),
            pages: InplaceList::new(),
        };
        res
    }

    pub fn allocate(&mut self) -> Option<NonZero<*mut u8>> {
        // FIXME: horrible hack
        let self2 = unsafe { reborrow_mut!(self, Self) };

        if let Some(node) = self.frames.first_mut() {
            unsafe { self2.frames.remove(node); }

            let res = node as *mut Node as *mut u8;
            debug_assert!(res as usize != 0);
            let res = unsafe { NonZero::new(res) };

            unsafe { Self::get_page_node(res).as_mut().inc() };
            debug_assert!(
                unsafe { Self::get_page_node(res).as_ref().allocated() <=
                    (PAGE_SIZE - size_of::<PageNode>()) / self.frame_size }
            );

            return Some(res);
        }

        // we have to allocate new page
        tryo!(self.allocate_page());
        debug_assert!(self.frames.first_mut().is_some());
        self.allocate()  // recursion
    }

//    pub unsafe fn deallocate(&mut self, address: *mut u8) {
//        let page_node = Self::get_page_node(address);
//        let node = Self::get_node(address);
//        self.frames.insert();
//    }

    fn allocate_page(&mut self) -> Option<()> {
        let page = tryo!(Page::allocate());
        let page_start = *page.ptr();

        let mut cur_ptr = unsafe {
            ptr::write(page_start as *mut PageNode, PageNode::new(page));
            self.pages.insert(&mut *(page_start as *mut PageNode));
            page_start.offset(size_of::<PageNode>() as isize)
        };

        while PAGE_SIZE - ((cur_ptr as usize) - (page_start as usize)) >= self.frame_size {
            cur_ptr = unsafe {
                ptr::write(cur_ptr as *mut Node, Node::new(()));
                self.frames.insert(&mut *(cur_ptr as *mut Node));
                cur_ptr.offset(self.frame_size as isize)
            }
        }
        Some(())
    }

//    unsafe fn get_node(ptr: *mut u8) -> &'static mut Node {
//
//    }

    unsafe fn get_page_node(ptr: NonZero<*mut u8>) -> &'static mut PageNode {
        // page node is in the begin of a page
        unsafe { &mut *(utility::round_down(*ptr as usize, PAGE_SIZE) as *mut PageNode) }
    }
}


#[cfg(os_test)]
pub mod slab_tests {
    use super::*;
    use super::{ Node, PageNode };
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
        const SIZE: usize = 128;

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
