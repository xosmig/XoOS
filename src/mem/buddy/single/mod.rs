
mod list;

use ::core::ptr::{self, Shared};
use ::mem::paging::PAGE_SIZE;
use ::mem::*;
use ::mem;
use ::utility::*;
use ::core::slice;

use self::list::*;

const MIN_SIZE: usize = PAGE_SIZE * 10;
const MAX_HEIGHT: usize = 32;
const PADDING: usize = 8;


pub struct Single {
    heads: [Node; MAX_HEIGHT],
    height: usize,
    nodes: &'static mut [Node],
    first_page: *mut u8,
}

impl Single {
    pub unsafe fn new(entry: &memory_map::Entry) -> Option<&'static mut Self> {
        let end = round_down(entry.end() as usize, PAGE_SIZE);
        let begin = round_up(entry.start() as usize, PADDING);
        if !entry.is_available() || begin + MIN_SIZE > end {
            return None;  // too small memory region
        }

        // the number of pages
        let cnt = (end - begin - size_of::<Single>()) / (PAGE_SIZE + size_of::<Node>());
        let height = log2_floor(cnt) + 1;

        debug_assert!(height < MAX_HEIGHT);
        debug_assert!(cnt >= 3);

        let first_node = (begin + size_of::<Single>()) as *mut Node;
        let first_page = (end - PAGE_SIZE * cnt) as *mut u8;

        ptr::write(
            begin as *mut _,
            Single {
                heads: generate![Node::new(); MAX_HEIGHT],
                height: height,
                nodes: slice::from_raw_parts_mut(first_node, cnt),
                first_page: first_page
            }
        );
        let mut it = &mut*(begin as *mut Single);

        // initialize nodes
        for i in 0..(cnt as isize) {
            ptr::write(first_node.offset(i), Node::new());
            it.heads[0].insert(&mut*first_node.offset(i));
        }

        debug_assert!(first_node.offset(cnt as isize) as usize <= first_page as usize);



        // TODO: try_go_up many times for every Node


        Some(it)
    }
}
