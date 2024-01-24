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
        n: usize,
        m: usize,
        mut aabb: [(usize, usize); m],
    };
    // https://atcoder.jp/contests/arc126/editorial/2626
    aabb.sort_unstable_by(|x, y| x.0.cmp(&y.0).then(y.1.cmp(&x.1)));

    let mut ls = ac_library::LazySegtree::<MaxMonoid>::new(n + 1);
    let mut dp = vec![0; n + 1];
    for (a, b) in aabb.iter().copied() {
        let x = ls.prod(0..b) + 1;
        dp[a] = x;
        ls.set(b, x);
    }
    let rs = ls.all_prod();
    println!("{rs}");
}
