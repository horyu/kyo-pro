#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

use ac_library::{Additive, MapMonoid, Monoid};
struct MM;
impl MapMonoid for MM {
    type M = Additive<i32>;
    type F = i32;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        llrr: [(Usize1, Usize1); m],
    };
    let mut ls = ac_library::LazySegtree::<MM>::new(n);
    // TODO

    // println!("{rs}");
}
