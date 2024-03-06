# Architecture
**Contents**
1. Data Structures
  - A. Sequences: `Seq`, `Heap`
  - B. Lookups: `Set`, `Map`
  - C. Graphs: `List`, `Tree`, `Graph`
  - D. Linear Algebra: `Nat`, `Vector`, `Matrix`, `Tensor`
  - _. References
2. Design Points
  - A. Correctness
  - B. Performance

## A. Sequences: `Seq`, `Heap`
## B. Lookups: `Set`, `Map`
## C: Graphs: `List`, `Tree`, `Graph` 

`List`
`Tree`
`Graph`

## D: `Linear Algebra`
- https://lib.rs/crates/num
- https://lib.rs/crates/ndarray

## _. References
**TAOCP + CLRS Alternatives**
- Kleinburg and Tardos
- Erickson
- DPV

**Intractable Problems**
- A Guide to Algorithmic Lower Bounds Demaine, Gasarch, Hajiaghayi
- Games, Puzzles, and Computation (Hearn, Demaine)
- Computers and Intractability (Garey, Johnson)
- Complexity Zoo (Aaronson)
- Limits to Parallel Computation: Greenlaw, Hoover, Ruzzo

# 2. Design Points

## A. Correctness
Program correctness lies on a gradient. Point-wise specification with unit tests
can be improved with lightweight formal methods that explicitly explore semantic
spaces. Kakashi uses refinement types with `flux`, property-based testing with
`proptest`, and model checking with `kani`. As of the time writing (early 2024),
interactive proof assistants don't have extraction support for Rust. Any stronger
guarantees that are needed will be lifted to the static level with type-level
programming via Rust's trait induction (similar to Haskell and Scala), which is
implemented by rustc's logic solver `chalk`.

```
complexity
^
|                                                                 deductive
|                                                                 verification
|                                                         model
|                                                         checking
|                                            type-level
|                                            programming
|                               property
|                               based
|                               testing
|                                    fuzzing/chaos (machines)
|                                    pen-testing (humans)
|                      
|                               heapless
|                               data
|                               structures
|
|                         safe
|                         rust
|         micro
|         benchmarks
|
|  unit tests
|
-------------------------------------------------------------------------> correctness
```

**References**
  - Harper: [The Holy Trinity](https://existentialtype.wordpress.com/2011/03/27/the-holy-trinity/)
  - Wadler: [Propositions as Types](https://www.youtube.com/watch?v=IOiZatlZtGU)
  - Ragde: [Logic and Computation Intertwined](https://cs.uwaterloo.ca/~plragde/flane/LACI/)
  - Kolomatski: [Proof, Programming, and Type Theory](https://www.math.stonybrook.edu/~tkolomat/mat250-sum20/Notes/notes250.html)

## B. Performance
```
complexity
^
|                                                                 
|                                                                 
|                                                         
|                                                         
|                       
|
|                               
|                               
|                                                   SIMD
|                                                   instructions
|                                    
|                                   memory
|                                   optimization
|                               
|                               
|                           parallel
|                           loops
|                         
|               non-GC
|               lang
|         
|  GC
|  lang
-------------------------------------------------------------------------> speed
```

## C. References