
mod list;

use ::core::ptr::{self, Shared};
use ::mem::*;
use ::mem::paging::PAGE_SIZE;
use ::utility::*;
use ::core::slice;

use self::list::*;

const MIN_SIZE: usize = PAGE_SIZE * 10;
const MAX_HEIGHT: usize = 32;
const PADDING: usize = 8;


pub struct Single {
    lists: [List; MAX_HEIGHT],
    height: usize,
    nodes: &'static mut [Node],
    first_page: *mut u8,
}

impl Single {
    pub unsafe fn new(begin: usize, end: usize) -> Option<&'static mut Self> {
        let begin = round_up(begin, PADDING);
        let end = round_down(end, PAGE_SIZE);
        if begin + MIN_SIZE > end {
            return None;  // too small memory region
        }

        // the number of pages
        let cnt = (end - begin - size_of::<Single>()) / (PAGE_SIZE + size_of::<Node>());
        let height = log2_floor(cnt) + 1;

        debug_assert!(height < MAX_HEIGHT);
        debug_assert!(cnt >= 3);

        let first_node = (begin + size_of::<Single>()) as *mut _;
        // initialize nodes
        let icnt = cnt as isize;
        for i in 0..icnt {
            ptr::write(
                first_node.offset(i),
                Node {
                    next: Shared::new(first_node.offset(if i == icnt - 1 {0} else {i + 1})),
                    prev: Shared::new(first_node.offset(if i == 0 {icnt - 1} else {i - 1})),
                    occupied: false,
                }
            );
        }

        let first_page = (end - PAGE_SIZE * cnt) as *mut _;
        debug_assert!(first_node.offset(icnt) as usize <= first_page as usize);

        ptr::write(
            begin as *mut _,
            Single {
                lists: Default::default(),
                height: height,
                nodes: slice::from_raw_parts_mut(first_node, cnt),
                first_page: first_page
            }
        );

        // TODO: try_go_up many times for every Node

        Some(&mut*(begin as *mut _))
    }
}
