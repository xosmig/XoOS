
use ::core::ptr::{ self, Shared };

pub struct Node {
    next: Option<Shared<Node>>,
    prev: Option<Shared<Node>>,
    num: u32,
    level: u16,
    occupied: bool,
}

impl Node {
    pub const fn new(num: usize) -> Self {
        Node { next: None, prev: None, num: num as u32, level: 0, occupied: false }
    }

    pub fn num(&self) -> usize {
        self.num as usize
    }

    pub fn level(&self) -> usize {
        self.level as usize
    }

    pub fn set_free(&mut self) {
        self.occupied = false;
    }

    pub fn set_occupied(&mut self) {
        self.occupied = true;
    }

    pub fn get_buddy(&self) -> usize {
        self.buddy_on_level(self.level as usize)
    }

    pub fn buddy_on_level(&self, level: usize) -> usize {
        self.num() ^ (1 << level)
    }

    pub fn is_free(&self) -> bool {
        !self.is_occupied()
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level as u16;
    }

    pub fn ready(&self, buddy: &Node) -> bool {
        let ret = self.is_free() && self.level() == buddy.level();
        if ret {
            debug_assert!(buddy.num() == self.get_buddy());
        }
        ret
    }
}


pub struct List {
    first: Option<Shared<Node>>,
}

impl List {
    pub const fn new() -> Self {
        List { first: None }
    }

    pub fn remove(&mut self, node: &mut Node) {
        if let Some(ptr) = self.first {
            if *ptr == node {
                self.first = node.next;
            }
        }

        if let Some(prev) = node.prev {
            unsafe { (**prev).next = node.next };
        }
        if let Some(next) = node.next {
            unsafe { (**next).prev = node.prev };
        }
        node.next = None;
        node.prev = None;
    }

    /// insert node into the first place in list
    pub fn insert(&mut self, node: &mut Node) {
        unsafe {
            // change links in `other`
            node.prev = None;
            node.next = self.first;

            self.first = Some(Shared::new(node));
            // note: next is already updated (former self.first)
            if let Some(second) = node.next {
                unsafe { (**second).prev = self.first };
            }
        }
    }

    pub fn first(&mut self) -> Option<usize> {
        self.first.map(|x| unsafe { (**x).num() })
    }

    /*pub fn iter_mut(&mut self) -> IterMut {
        IterMut { node: self.first }
    }*/
}


/*pub struct IterMut {
    node: Option<Shared<Node>>,
}

impl Iterator for IterMut {
    type Item = &'static mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.map(|x| unsafe { &mut**x }) {
            self.node = node.next;
            Some(node)
        } else {
            None
        }
    }
}*/
