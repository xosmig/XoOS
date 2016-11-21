
prelude!();

pub struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    object: T,
}

impl<T> PartialEq<Node<T>> for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        (self as *const _) == (other as *const _)
    }
}

impl<T> Node<T> {
    /// Creates a `Node` with given object, without left and right pointers.
    pub const fn new(object: T) -> Self {
        Node { next: None, prev: None, object: object }
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.object
    }

    pub fn as_ref(&self) -> &T {
        &self.object
    }

    pub fn prev(&self) -> Option<&Node<T>> {
        self.next.map(|x| unsafe { &**x })
    }

    pub fn prev_mut(&mut self) -> Option<&mut Node<T>> {
        self.next.map(|x| unsafe { &mut **x })
    }
}


pub struct InplaceList<T> {
    first: Option<Shared<Node<T>>>,
}

/// Provides some kind of double linked list without memory allocator.
/// In order to add a `Node` to the list one have to create it themselves and use `insert` method.
impl<T> InplaceList<T> {
    /// Crates an empty `InplaceList`.
    pub const fn new() -> Self {
        InplaceList { first: None }
    }

    /// Remove an element by its iterator.
    pub unsafe fn remove(&mut self, node: &mut Node<T>) {
        if let Some(ptr) = self.first {
            if *ptr as *const _ == node as *const _ {
                self.first = node.next;
            }
        }

        if let Some(prev) = node.prev {
            (**prev).next = node.next;
        }
        if let Some(next) = node.next {
            (**next).prev = node.prev;
        }
        node.next = None;
        node.prev = None;
    }

    /// insert node into the first place in list
    pub unsafe fn insert(&mut self, node: &mut Node<T>) {
        // change links in `other`
        node.prev = None;
        node.next = self.first;

        self.first = Some(Shared::new(node));
        // note: next is already updated (former self.first)
        if let Some(second) = node.next {
            unsafe { (**second).prev = self.first };
        }

        debug_assert!(match node.next { Some(shared) => *shared != node, None => true });
    }

    pub fn last(&self) -> Option<&Node<T>> {
        self.first.map(|x| unsafe { &(**x) })
    }

    pub fn last_mut(&self) -> Option<&mut Node<T>> {
        self.first.map(|x| unsafe { &mut (**x) })
    }

    /*pub unsafe fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { node: self.first }
    }*/
}


/*pub struct IterMut<T> {
    node: Option<Shared<Node<T>>>,
}

impl<T: 'static> Iterator for IterMut<T> {
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.map(|x| unsafe { &mut**x }) {
            self.node = node.next;
            Some(&mut node.object)
        } else {
            None
        }
    }
}*/
