#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use ac_library::{LazySegtree, MapMonoid, Max};
struct MaxMonoid;
impl MapMonoid for MaxMonoid {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &usize, &x: &usize) -> usize {
        f.max(x)
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        f.max(g)
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        llrr: [(Usize1, Usize1); n],
    };
    let mut ls = LazySegtree::<MaxMonoid>::new(5e5 as usize);
    for (l, r) in llrr {
        let rs = ls.prod(l..=r) + 1;
        ls.apply_range(l..=r, rs);
        println!("{rs}");
    }
}
