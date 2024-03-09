use crate::num::Nat;

#[derive(Default, Clone, PartialEq)]
pub enum List<V> {
    #[default]
    None,
    Some(V, Box<Self>),
}

impl<V> List<V> {
    // constructors
    fn new() -> List<V> {
        todo!()
    }

    fn append(&self, other: List<V>) -> List<V> {
        todo!()
    }

    // accessors
    fn first(&self) -> V {
        todo!()
    }

    fn rest(&self) -> List<V> {
        todo!()
    }

    // queries
    fn exists(t: V) -> bool {
        todo!()
    }

    fn length(&self) -> Nat {
        todo!()
    }
}

impl<V> std::fmt::Debug for List<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Some(arg0, arg1) => f.debug_tuple("Some").field(arg0).field(arg1).finish(),
        }
    }
}

impl<V> IntoIterator for List<V>
where
    V: Ord,
{
    type Item;

    type IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
