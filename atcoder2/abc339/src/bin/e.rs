#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
struct MaxMonoid;
impl MapMonoid for MaxMonoid {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        d: usize,
        aa: [usize; n],
    };
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::new(5e5 as usize * 2 + 1);

    for a in aa {
        let x = ls.prod(a.saturating_sub(d)..=(a + d));
        ls.set(a, x + 1);
    }

    let rs = ls.all_prod();
    println!("{rs}");
}
