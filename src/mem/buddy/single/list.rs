
use ::core::ptr::{ self, Shared };
use ::mem;

pub struct Node {
    next: Option<Shared<Node>>,
    prev: Option<Shared<Node>>,
    occupied: bool,
}

impl Node {
    pub const fn new() -> Self {
        Node { next: None, prev: None, occupied: false }
    }

    pub fn insert(&mut self, other: &mut Node) {
        unsafe {
            // remove other from a previous list
            {
                if let Some(node) = other.prev {
                    (**node).next = other.next;
                }
                if let Some(node) = other.next {
                    (**node).prev = other.prev;
                }
            }

            // change links in other
            other.prev = Some(Shared::new(self));
            other.next = self.next;

            self.next = Some(Shared::new(other));
            if let Some(node) = other.next {
                (**node).prev = self.next;
            }
        }
    }
}

#[derive(Default)]
pub struct List {
    head: Option<Shared<Node>>,
}
