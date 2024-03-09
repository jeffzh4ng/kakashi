use crate::num::Nat;

#[derive(Clone, PartialEq)]
pub enum Tree<V> {
    Leaf(V),
    Node(V, Option<Box<Self>>, Option<Box<Self>>),
}

// constructor:
// - new
// - insert
// - union

// queries:
// - size
// - search
// - children
// - equals

impl<V> Tree<V> {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, v: V) -> Self {
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

    fn deep_eq(&self, other: Self) -> bool {
        todo!()
    }
}

impl<V> std::fmt::Debug for Tree<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(arg0) => f.debug_tuple("Leaf").field(arg0).finish(),
            Self::Node(arg0, arg1, arg2) => f
                .debug_tuple("Node")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}

impl<V> Default for Tree<V> {
    fn default() -> Self {
        todo!()
    }
}

impl<V> IntoIterator for Tree<V>
where
    V: Ord,
{
    type Item;

    type IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
