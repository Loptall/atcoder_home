#![allow(unused_imports)]

use std::cmp::{
    max, min, Ordering,
    Ordering::{Equal, Greater, Less},
    Reverse,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::mem::swap;
use std::num::Wrapping;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};
use std::process::exit;
use std::{f32, f64, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

use itertools::Itertools as _;
use itertools_num::ItertoolsNum as _;
use maplit;
use num_bigint::{BigInt, BigUint};
use num_complex::Complex;
use num_integer::{binomial, gcd, lcm, multinomial, Integer};
use num_rational::Rational;
use num_traits::{
    clamp, one, pow, zero, Num, NumAssignOps, NumOps, One, Pow, Signed, Unsigned, Zero,
};
use permutohedron::Heap;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use rand::{random, rngs::SmallRng, Rng, SeedableRng};

use ac_library_rs::{
    convolution, convolution_i64, crt, floor_sum, inv_mod, lcp_array, lcp_array_arbitrary, pow_mod,
    suffix_array, suffix_array_arbitrary, suffix_array_manual, z_algorithm, z_algorithm_arbitrary,
    Additive, Dsu, Edge, FenwickTree, LazySegtree, MapMonoid, Max, MfGraph, Min, MinCostFlowGraph,
    ModInt, ModInt1000000007, ModInt998244353, Monoid, Multiplicative, SccGraph, Segtree, TwoSat,
};

#[macro_use]
extern crate argio as _;

pub use consts::*;
pub mod consts {
    pub const MOD10E9_7: usize = 1000000007; // 10 ^ 9 + 7
    pub const MOD99_: usize = 998244353;
    pub const MAX: usize = std::usize::MAX; // = 2 ^ 64 - 1 = 18446744073709551615 ≈ 1.8 * 10 ^ 19
    pub const INF: usize = 2000000000000000000; // MAX / 9 < 2 * 10e18 < MAX / 10
    pub const FNI: i64 = -2000000000000000000; // == -(INF as i64)
    pub const PI: f64 = std::f64::consts::PI; // 3.141592653589793 -- 10 ^ -15
    pub const ASCII_A_UPPER: u8 = 65;
    pub const ASCII_A_LOWER: u8 = 97;
    pub const ASCII_ZERO: u8 = 48;
    pub const ADJ4: &[(isize, isize); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    pub const ADJ8: &[(isize, isize); 8] = &[
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
}

#[argio]
fn main() {
    todo!("solve it!");
}

