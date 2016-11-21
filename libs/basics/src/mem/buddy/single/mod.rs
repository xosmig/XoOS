
prelude!();

mod entry;

use ::mem::paging::PAGE_SIZE;
use ::mem::{ get_mut_ptr, memory_map };
use ::utility::{ round_up, round_down, log2_floor };
use ::core::slice;
use ::core::cmp::{min, max};

use self::entry::*;
use super::super::inplace_list::*;


const MIN_SIZE: usize = PAGE_SIZE * 10;
const MAX_HEIGHT: usize = 32;
const PADDING: usize = 8;

pub struct Single {
    lists: [InplaceList<Entry>; MAX_HEIGHT],
    height: usize,
    nodes: &'static mut [Node<Entry>],
    first_page: *mut u8,
    length: usize,
}

impl Single {
    pub unsafe fn new(entry: &memory_map::Entry) -> Option<&'static mut Self> {
        let end = round_down(entry.end() as usize, PAGE_SIZE);
        let begin = round_up(entry.start() as usize, PADDING);
        if !entry.is_available() || begin + MIN_SIZE > end {
            return None;  // too small memory region
        }

        // the number of pages
        let pages_cnt = (end - begin - size_of::<Single>()) / (PAGE_SIZE + size_of::<Node<Entry>>());
        let height = log2_floor(pages_cnt) + 1;
        debug_assert!(height < MAX_HEIGHT);
        debug_assert!(pages_cnt >= 3);

        let first_node: *mut Node<Entry> = get_mut_ptr(begin + size_of::<Single>());
        let first_page: *mut u8 = get_mut_ptr(end - PAGE_SIZE * pages_cnt);
        debug_assert!(first_node.offset(pages_cnt as isize) as usize <= first_page as usize);

        ptr::write(
            get_mut_ptr(begin),
            Single {
                lists: generate![InplaceList::new(); MAX_HEIGHT],
                height: height,
                nodes: slice::from_raw_parts_mut(first_node, pages_cnt),
                first_page: first_page,
                length: pages_cnt * PAGE_SIZE,
            }
        );
        let mut it: &mut Single = &mut*(get_mut_ptr(begin));

        // initialize nodes
        for i in 0..pages_cnt {
            let ptr = first_node.offset(i as isize);
            ptr::write(ptr, Node::new(Entry::new(i)));
            it.lists[0].insert(&mut*ptr);
        }

        for i in 0..height {
            for i in (0..pages_cnt).step_by(1 << (i + 1)) {
                if it.nodes[i].as_ref().is_free() {
                    it.go_up_once(i);
                }
            }
        }

        Some(it)
    }

    pub fn allocate(&mut self, req_level: usize) -> Option<NonZero<*mut u8>> {
        unsafe {
            for lvl in req_level..self.height {
                if let Some(num) = self.lists[lvl].last().map(|x| x.as_ref().num()) {
                    debug_assert!(self.nodes[num].as_ref().is_free());
                    self.down_to_level(num, req_level);
                    {
                        let node = &mut self.nodes[num];
                        node.as_mut().set_occupied();
                        self.lists[node.as_ref().level()].remove(node);
                    }
                    return Some(unsafe { NonZero::new(self.node_to_ptr(num)) });
                }
            }
        }

        None  // not found
    }

    pub unsafe fn deallocate(&mut self, ptr: NonZero<*mut u8>) {
        let mut num = self.ptr_to_node(*ptr);
        {
            let node = &mut self.nodes[num];
            assert!(node.as_ref().is_occupied(), "Invalid buddy deallocate call on {:?}", *ptr);
            node.as_mut().set_free();
            self.lists[node.as_ref().level()].insert(node);
        }
        while let Some(next) = self.go_up_once(num) {
            num = next;
        }
    }

    pub fn contains_addr(&self, ptr: *mut u8) -> bool {
        let addr = ptr as usize;
        let begin = self.first_page as usize;
        addr >= begin && addr <= begin + self.length
    }

    /// Returns None if it can't be moved on the next level.
    /// Otherwise returns a number of the main node in the pair on the next level.
    unsafe fn go_up_once(&mut self, node: usize) -> Option<usize> {
        debug_assert!(self.nodes[node].as_ref().is_free());

        let (level, buddy) = {
            let entry = self.nodes[node].as_ref();
            let buddy = entry.get_buddy();

            if buddy >= self.nodes.len() || !self.nodes[buddy].as_ref().ready(entry) {
                return None;
            }

            (entry.level(), buddy)
        };

        debug_assert!(level + 1 < self.height);

        let major = min(node, buddy);
        let minor = max(node, buddy);

        let (left, right) = self.nodes.split_at_mut(minor);
        let major = &mut left[major];
        let minor = &mut right[0];

        self.lists[level].remove(major);
        self.lists[level].remove(minor);

        minor.as_mut().set_occupied();
        major.as_mut().set_level(level + 1);

        self.lists[level + 1].insert(major);

        Some(major.as_ref().num())
    }

    unsafe fn go_down_once(&mut self, node: usize) {
        debug_assert!(self.nodes[node].as_ref().is_free());

        let level = self.nodes[node].as_ref().level();
        let buddy = self.nodes[node].as_ref().buddy_on_level(level - 1);
        assert!(level > 0);
        debug_assert!(node < buddy);
        debug_assert!(self.nodes[buddy].as_ref().is_occupied());

        let (left, right) = self.nodes.split_at_mut(buddy);
        let node = &mut left[node];
        let buddy = &mut right[0];

        self.lists[level].remove(node);

        buddy.as_mut().set_free();
        node.as_mut().set_level(level - 1);
        buddy.as_mut().set_level(level - 1);

        self.lists[level - 1].insert(node);
        self.lists[level - 1].insert(buddy);
    }

    unsafe fn down_to_level(&mut self, node: usize, req_level: usize) {
        while self.nodes[node].as_ref().level() > req_level {
            self.go_down_once(node);
        }
    }

    fn node_to_ptr(&self, node_num: usize) -> *mut u8 {
        unsafe { self.first_page.offset((node_num * PAGE_SIZE) as isize) }
    }

    fn ptr_to_node(&self, ptr: *mut u8) -> usize {
        let diff = (ptr as usize) - (self.first_page as usize);
        debug_assert!(diff % 4096 == 0);
        debug_assert!(diff / 4096 < self.nodes.len());
        diff / 4096
    }
}

