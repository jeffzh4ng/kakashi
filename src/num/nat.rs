use super::Num;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Nat {
    #[default]
    Z,
    S(Box<Nat>),
}

impl Num for Nat {
    fn plus(n: Self, m: Self) -> Self {}

    fn sub(n: Self, other: Self) -> Self {
        todo!()
    }

    fn mult(n: Self, other: Self) -> Self {
        todo!()
    }

    fn div(n: Self, other: Self) -> Self {
        todo!()
    }
}
