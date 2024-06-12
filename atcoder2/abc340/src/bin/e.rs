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
        m: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::from(aa);
    for b in bb {
        let mut cnt = ls.get(b);
        ls.set(b, 0);
        let from = (b + 1) % n;

        ls.apply_range(.., cnt / n);
        cnt %= n;
        ls.apply_range(from..(n.min(from + cnt)), 1);
        cnt = cnt.saturating_sub(n - from);
        ls.apply_range(..cnt, 1);
    }

    let rs = (0..n).map(|i| ls.get(i)).join(" ");
    println!("{rs}");
}
