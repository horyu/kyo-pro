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
    type M = Max<isize>;
    type F = isize;

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
        w: usize,
        n: usize,
        llrrvv: [(usize, usize, isize); n],
    };
    let mut vv = vec![-1isize; w + 1];
    vv[0] = 0;
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::from(vv);
    for (l, r, v) in llrrvv {
        for i in (0..=w).rev() {
            let x = ls.prod(i.saturating_sub(r)..(i + 1).saturating_sub(l));
            if x != -1 {
                let y = ls.get(i);
                if y < x + v {
                    ls.set(i, x + v);
                }
            }
        }
    }
    let rs = ls.get(w);
    println!("{rs}");
}
