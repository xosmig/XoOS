
use ::core::ptr::Shared;

pub struct Node {
    pub next: Shared<Node>,
    pub prev: Shared<Node>,
    pub occupied: bool,
}

#[derive(Default)]
pub struct List {
    head: Option<Shared<Node>>,
}
