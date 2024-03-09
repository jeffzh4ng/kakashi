pub enum Nat {
    Z,
    S(Box<Nat>),
}
