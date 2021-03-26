//! # Hilbert-Index
//! 
//! D-dimensional Hilbert curve for Rust.
//! 
//! 
//! ## Requirements
//! 
//! This crate requires Rust 1.51 or later, due to [const-generics](https://rust-lang.github.io/rfcs/2000-const-generics.html).
//! Const-generics enables us to use `[usize; D]` instead of `Vec<usize>`.
//! 
//! 
//! ## Features
//! 
//! This crate gives conversion between `usize` (Hilbert indices) and `[usize; D]` (grid points), 
//! based on the D-dimensional [Hilbert curve](https://en.wikipedia.org/wiki/Hilbert_curve). 
//! A Hilbert curve fills all grid points in a D-dimensional cube,
//! and can be used for D-dimensional data structures, such as [Hilbert R-tree](https://en.wikipedia.org/wiki/Hilbert_R-tree).
//! 
//! A `D`-dimensional Hilbert curve with level (order) `l` is a map from indices `0..2.pow(D*l)` to grid points `[usize; D]`,
//! whose component `x` satisfy `0 <= x < 2.pow(l)`.
//! Adjacent indices give adjacent grid points.
//! Input outside the range is not supported and may cause unexpected results.
//! 
//! The implemented algorithm is based on Chris Hamilton's report, 
//! "[Compact Hilbert Indices](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.133.7490&rep=rep1&type=pdf)".
//! See also [Compact Hilbert indices: Space-filling curves for domains with unequal side lengths](https://doi.org/10.1016/j.ipl.2007.08.034).
//! 
//! 
//! ## Usage
//! 
//! This crate provides 2 traits, `FromHilbertIndex` and `ToHilbertIndex`.
//! Additionally, `indices` function gives an iterator that generates all Hilbert indices.
//! 
//! Convert a index to a grid point.
//! 
//! ```rust
//! use hilbert_index::FromHilbertIndex;
//! const D: usize = 3;
//! 
//! let level = 2;
//! for hindex in hilbert_index::indices::<D>(level) {
//!     let position: [usize; D] = hindex.from_hilbert_index(level);
//!     println!("p[{:02}] = {:?}", hindex, position);
//! }
//! ```
//! 
//! You can also use `from_hindex` instead of `from_hilbert_index`.
//! 
//! 
//! Convert a grid point to a Hilbert index.
//! 
//! ```rust
//! use hilbert_index::ToHilbertIndex;
//! 
//! let level = 1;
//! assert_eq!( 0, [ 0, 0, 0 ].to_hilbert_index(level));
//! assert_eq!( 1, [ 0, 1, 0 ].to_hilbert_index(level));
//! assert_eq!( 2, [ 0, 1, 1 ].to_hilbert_index(level));
//! assert_eq!( 3, [ 0, 0, 1 ].to_hilbert_index(level));
//! assert_eq!( 4, [ 1, 0, 1 ].to_hilbert_index(level));
//! assert_eq!( 5, [ 1, 1, 1 ].to_hilbert_index(level));
//! assert_eq!( 6, [ 1, 1, 0 ].to_hilbert_index(level));
//! assert_eq!( 7, [ 1, 0, 0 ].to_hilbert_index(level));
//! ```
//! 
//! You can also use `to_hindex` instead of `to_hilbert_index`.
//! 
//! 
//! ## Similar crates
//! 
//! * [hilbert](https://crates.io/crates/hilbert)
//! * [hilbert_2d](https://crates.io/crates/hilbert_2d) (only for 2D)
//! * [hilbert_curve](https://crates.io/crates/hilbert_curve) (only for 2D)
//! * [fast_hilbert](https://crates.io/crates/fast_hilbert) (only for 2D)
//! 

/// Get an iterator that generates all Hilbert indices for a given level.
/// 
/// The return value is equal to `0..2usize.pow((D*level) as u32)`.
pub fn indices<const D: usize>(level: usize) -> impl std::iter::Iterator<Item=usize> {
    0..2usize.pow((D*level) as u32)
}

// 基本格子における部分格子の数 2^D - 1
const fn max<const D: usize>() -> usize { !( {std::usize::MAX}<<D ) }

// Gray code
#[inline]
fn gc(i: usize) -> usize { i^(i >> 1) }

// Gray code の逆変換.
#[inline]
fn gc_inv<const D: usize>(g: usize) -> usize { (1..D).fold(g, |i, j| i^(g>>j)) }

#[inline]
fn g(i: usize) -> usize {
    (!i).trailing_zeros() as usize
}

#[inline]
fn dmap<const D: usize>(i: usize) -> usize {
    if i == 0 { 0 } else if i&1 == 0 { g(i-1) % D } else { g(i) % D }
}

#[inline]
fn emap(i: usize) -> usize {
    if i == 0 { 0 } else { gc(2*( (i-1)/2 )) }
}

// #[inline]
// #[allow(dead_code)]
// fn fmap<const D: usize>(i: usize) -> usize { emap(i)^(1 << dmap::<D>(i)) }

// D bit の範囲で右回転
#[inline]
fn rotate_right<const D: usize>(b: usize, i: usize) -> usize {
    let i = i.rem_euclid(D);
    (b >> i)^(b << (D-i))&max::<D>()
}

// D bit の範囲で左回転
#[inline]
fn rotate_left<const D: usize>(b: usize, i: usize) -> usize {
    let i = i.rem_euclid(D);
    max::<D>() & (b << i)^(b >> (D-i))
}

#[inline]
fn t<const D: usize>(b: usize, e: usize, d: usize) -> usize { rotate_right::<D>(b^e, d+1) }

#[inline]
fn t_inv<const D: usize>(b: usize, e: usize, d: usize) -> usize { rotate_left::<D>(b, d+1)^e }

#[inline]
fn reduce<const D: usize>(p: &[usize; D], i: usize) -> usize {
    p.iter().enumerate()
        .fold(0, |l, (k, p)| l^( ((p >> i)&1) << k))
}

pub fn offset<const D: usize>(level: usize) -> usize {
    (0..level).fold(0, |ofs, _| {
        (ofs << D) | 1
    })
}

/// Convert `[usize; D]` to `usize`.
pub trait ToHilbertIndex<const D: usize> {
    /// Convert a grid point `[usize; D]` to a Hilbert index `usize`.
    fn to_hilbert_index(&self, level: usize) -> usize;

    /// Equivalent to `to_hilbert_index` (abbreviation).
    fn to_hindex(&self, level: usize) -> usize {
        self.to_hilbert_index(level)
    }
}

/// Convert `usize` to `[usize; D]`.
pub trait FromHilbertIndex<const D: usize> {
    /// Convert a Hilbert index `usize` to a grid point `[usize; D]`.
    fn from_hilbert_index(&self, level: usize) -> [usize; D];

    /// Equivalent to `from_hilbert_index` (abbreviation).
    fn from_hindex(&self, level: usize) -> [usize; D] {
        self.from_hilbert_index(level)
    }
}

impl<const D: usize> ToHilbertIndex::<D> for [usize; D] {
    fn to_hilbert_index(&self, level: usize) -> usize {
        let (mut h, mut e, mut d) = (0, 0, 0);
        for i in(0..level).rev() {
            let l = t::<D>(reduce(&self, i), e, d);
            let w = gc_inv::<D>(l);
            e = e^( rotate_left::<D>(emap(w), d+1) );
            d = ( d + dmap::<D>(w) + 1 )%D;
            h = (h << D) | w;
        }

        h
    }
}

impl<const D: usize> FromHilbertIndex::<D> for usize {
    fn from_hilbert_index(&self, level: usize) -> [usize; D] {
        let (mut e, mut d) = (0, 0);
        let mut p = [0; D];

        for i in (0..level).rev() {
            let w = (0..D).fold(0, |w, k| w^( ((self >> (i*D + k)) & 1 ) << k ));
            let l = t_inv::<D>(gc(w), e, d);
            for j in 0..D {
                p[j] = (p[j] << 1)|((l >> j)&1);
            }
            e = e^rotate_left::<D>( emap(w), d+1 );
            d = ( d + dmap::<D>(w) + 1 )%D;
        }

        p
    }
}

#[cfg(test)]
mod tests {
    use crate::{FromHilbertIndex, ToHilbertIndex};

    fn check<const D: usize>(level: usize) {
        //let max: usize = !( {std::usize::MAX}<<D );
        let max = 2usize.pow((D*level) as u32) - 1;

        for key in 0..2usize.pow((D*level) as u32) {
            let xyz: [usize; D] = key.from_hilbert_index(level);
            println!("p[{}] = {:?}", key, xyz);

            // 2 つの関数が正しく逆写像になっていることをチェック
            assert_eq!(key, xyz.to_hilbert_index(level)); 

            for x in xyz.iter() {
                // 各成分が正しく立方体の中に入っていることをチェック
                assert!((&0 <= x) && (x <= &max));
            }

            if key > 0 {
                // 直前の座標とひとつの値だけが \pm 1 違うことをチェック
                let prv: [usize; D] = (key-1).from_hilbert_index(level);

                let diff = prv.iter().zip(xyz.iter())
                    .map(|(&p, &c)| (p as isize - c as isize).abs())
                    .sum::<isize>();
                assert_eq!(diff, 1);
            }
        }
    }

    #[test]
    fn dim_two() {
        const D: usize = 2;
        for level in 1..8 { check::<D>(level); }
    }

    #[test]
    fn dim_three() {
        const D: usize = 3;
        for level in 1..7 { check::<D>(level); }
    }

    #[test]
    fn dim_four() {
        const D: usize = 4;
        for level in 1..6 { check::<D>(level); }
    }

    #[test]
    fn dim_five() {
        const D: usize = 5;
        for level in 1..5 { check::<D>(level); }
    }

    #[test]
    fn dim_six() {
        const D: usize = 6;
        for level in 1..4 { check::<D>(level); }
    }
}
