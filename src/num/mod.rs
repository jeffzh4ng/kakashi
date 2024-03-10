pub trait Num {
    fn plus(n: Self, other: Self) -> Self;
    fn sub(n: Self, other: Self) -> Self;
    fn mult(n: Self, other: Self) -> Self;
    fn div(n: Self, other: Self) -> Self;
}

pub mod int;
pub mod nat;
pub mod rat;
