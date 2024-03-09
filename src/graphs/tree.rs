use crate::num::Nat;

pub enum Tree<V> {
    Leaf(V),
    Node(V, Option<Box<Self>>, Option<Box<Self>>),
}

// constructor:
// - new
// - union

// queries:
// - size
// - search
// - children

impl<V> Tree<V> {
    fn new() -> Self {
        todo!()
    }

    fn union(&self, other: Self) -> Self {
        todo!()
    }

    fn size(&self) -> Nat {
        todo!()
    }

    fn search(&self, v: V) -> Option<Self> {
        todo!()
    }

    fn children(&self, v: V) -> (Option<Box<Self>>, Option<Box<Self>>) {
        todo!()
    }
}
