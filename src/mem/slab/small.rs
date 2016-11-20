
use ::prelude::*;
use mem::inplace_list::{ self, InplaceList };
use mem::buddy::*;
use mem::paging::PAGE_SIZE;

type Node = inplace_list::Node<()>;
type PageNode = inplace_list::Node<BuddyBox>;

// https://github.com/rust-lang/rfcs/issues/1144
/// Must be at least size_of::<Node>()
pub const MIN_FRAME_SIZE: usize = 16;
pub const MAX_FRAME_SIZE: usize = 256;

pub struct SmallSlabAllocator {
    frame_size: usize,
    frames: InplaceList<()>,
    head: Node,
    pages: InplaceList<BuddyBox>,
}

impl SmallSlabAllocator {
    /// `MIN_FRAME_SIZE` ≤ `frame_size` ≤ `MAX_FRAME_SIZE` .
    /// Use `BigSlabAllocator` for big frames.
    /// `frame_size` must be divisible by 8 for correct alignment.
    pub fn new(frame_size: usize) -> Self {
        assert!(frame_size <= MAX_FRAME_SIZE);
        assert!(frame_size >= MIN_FRAME_SIZE);
        assert!(frame_size % 8 == 0);

        let mut res = SmallSlabAllocator {
            frame_size: frame_size,
            frames: InplaceList::new(),
            head: Node::new(()),
            pages: InplaceList::new(),
        };
        unsafe { res.frames.insert(&mut res.head); }
        res
    }

    pub fn allocate(&mut self) -> Option<*mut u8> {
        if let Some(node) = self.head.next_mut() {
            unsafe { self.frames.remove(node); }
            return Some(node as *mut Node as *mut _);
        }

        // we have to allocate new page
        tryo!(self.allocate_page());
        self.allocate()  // recursion
    }

    /*pub unsafe fn deallocate(address: *mut u8) {

    }*/

    fn allocate_page(&mut self) -> Option<()> {
        let page = tryo!(unsafe { BuddyAllocator::get_instance().allocate_level(0) });
        let page_start = *page;
        let mut cur_ptr = unsafe {
            ptr::write(page_start as *mut PageNode, PageNode::new(page));
            page_start.offset(size_of::<PageNode>() as isize)
        };
        while PAGE_SIZE - ((cur_ptr as usize) - (page_start as usize)) >= self.frame_size {
            cur_ptr = unsafe {
                ptr::write(cur_ptr as *mut Node, Node::new(()));
                cur_ptr.offset(self.frame_size as isize)
            }
        }
        Some(())
    }
}

#[cfg(os_test)]
pub mod small_slab_tests {
    use super::*;
    use super::{ Node, PageNode };
    tests_module!("small_slab",
        min_frame_at_least_node_size,
        simple_allocate_test,
    );

    // it is necessary for correct alignment
    fn min_frame_at_least_node_size() {
        assert!(MIN_FRAME_SIZE >= size_of::<Node>());
    }

    fn simple_allocate_test() {
        let mut allocator = SmallSlabAllocator::new(16);
    }
}
