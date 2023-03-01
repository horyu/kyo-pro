#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use ac_library_rs::{Additive, LazySegtree, MapMonoid};
struct AddMonoid;
impl MapMonoid for AddMonoid {
    type M = Additive<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &usize, &x: &usize) -> usize {
        f + x
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        d: usize,
        a: usize,
        mut xxhh: [(usize, usize); n],
    };
    xxhh.sort_unstable();

    let mut ls = ac_library_rs::LazySegtree::<AddMonoid>::new(n);
    let mut rs = 0;
    for l in 0..n {
        let (lx, lh) = xxhh[l];
        let lc = ls.get(l);
        let add_cnt = lh.saturating_sub(lc * a).div_ceil(a);
        if add_cnt == 0 {
            continue;
        }
        rs += add_cnt;

        let r = xxhh.partition_point(|xh| xh.0 <= lx + 2 * d);
        ls.apply_range(l, r, add_cnt);
    }
    println!("{rs}");
}
