use crate::num::Nat;

pub enum List<T> {
    None,
    Some(T, Box<Self>),
}

impl<T> List<T> {
    // constructors
    fn new() -> List<T> {
        todo!()
    }

    fn append(&self, other: List<T>) -> List<T> {
        todo!()
    }

    // accessors
    fn first(&self) -> T {
        todo!()
    }

    fn rest(&self) -> List<T> {
        todo!()
    }

    // queries
    fn exists(t: T) -> bool {
        todo!()
    }

    fn length(&self) -> Nat {
        todo!()
    }
}
