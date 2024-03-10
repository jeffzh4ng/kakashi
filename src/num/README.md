# Num

## `Nat`

```rust
enum Nat {
    Z,
    S(Box<Nat>),
}
```

We define the universe of Nat inductively from the universe of Rust identifiers.

```
BNF notation
============
<nat> := Z
       | (S <nat>)

set notation
============
Z ∈ ℕ, Z ∉ of S[ℕ]
S: ℕ -> ℕ, S is injective
```

```
Prop: If (ℕ_1, Z_1, S_1) and  (ℕ_2, Z_2, S_2) are models of the natural numbers,
then there is a unique bijection f: ℕ_1, ℕ_2 such that f(Z_1) = z2 and
f(S_1(n)) = S_2(f(n)) for all ∈ ℕ_1.
```

## `Int`

## `Rat`