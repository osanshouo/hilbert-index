# Hilbert-Index

[![crates.io](https://img.shields.io/crates/v/hilbert_index?label=latest)](https://crates.io/crates/hilbert_index)
[![Documentation](https://docs.rs/hilbert_index/badge.svg)](https://docs.rs/hilbert_index/)

D-dimensional Hilbert curve for Rust.


## Requirements

This crate requires Rust 1.51 or later, due to [const-generics](https://rust-lang.github.io/rfcs/2000-const-generics.html).
Const-generics enables us to use `[usize; D]` instead of `Vec<usize>`.


## Features

This crate gives conversion between `usize` (Hilbert indices) and `[usize; D]` (grid points), 
based on the D-dimensional [Hilbert curve](https://en.wikipedia.org/wiki/Hilbert_curve). 
A Hilbert curve fills all grid points in a D-dimensional cube,
and can be used for D-dimensional data structures, such as [Hilbert R-tree](https://en.wikipedia.org/wiki/Hilbert_R-tree).

A `D`-dimensional Hilbert curve with level (order) `l` is a map from indices `0..2.pow(D*l)` to grid points `[usize; D]`,
whose component `x` satisfy `0 <= x < 2.pow(l)`.
Adjacent indices give adjacent grid points.
Input outside the range is not supported and may cause unexpected results.

The implemented algorithm is based on Butz's algorithm in Chris Hamilton's report, 
"[Compact Hilbert Indices](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.133.7490&rep=rep1&type=pdf)".
See also [Compact Hilbert indices: Space-filling curves for domains with unequal side lengths](https://doi.org/10.1016/j.ipl.2007.08.034).


## Usage

This crate provides 2 traits, `FromHilbertIndex` and `ToHilbertIndex`.
Additionally, `indices` function provides an iterator that generates all Hilbert indices.

Convert a index to a grid point.

```rust
use hilbert_index::FromHilbertIndex;
const D: usize = 3;

let level = 2;
for hindex in hilbert_index::indices::<D>(level) {
    let position: [usize; D] = hindex.from_hilbert_index(level);
    println!("p[{:02}] = {:?}", hindex, position);
}
```

You can also use `from_hindex` instead of `from_hilbert_index`.


Convert a grid point to a Hilbert index.

```rust
use hilbert_index::ToHilbertIndex;

let level = 1;
assert_eq!( 0, [ 0, 0, 0 ].to_hilbert_index(level));
assert_eq!( 1, [ 0, 1, 0 ].to_hilbert_index(level));
assert_eq!( 2, [ 0, 1, 1 ].to_hilbert_index(level));
assert_eq!( 3, [ 0, 0, 1 ].to_hilbert_index(level));
assert_eq!( 4, [ 1, 0, 1 ].to_hilbert_index(level));
assert_eq!( 5, [ 1, 1, 1 ].to_hilbert_index(level));
assert_eq!( 6, [ 1, 1, 0 ].to_hilbert_index(level));
assert_eq!( 7, [ 1, 0, 0 ].to_hilbert_index(level));
```

You can also use `to_hindex` instead of `to_hilbert_index`.


## Similar crates

* [hilbert](https://crates.io/crates/hilbert)
* [hilbert_2d](https://crates.io/crates/hilbert_2d) (only for 2D)
* [hilbert_curve](https://crates.io/crates/hilbert_curve) (only for 2D)
* [fast_hilbert](https://crates.io/crates/fast_hilbert) (only for 2D)
