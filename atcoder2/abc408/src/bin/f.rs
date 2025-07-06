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
        r: usize,
        i2h: [usize; n],
    };
    let mut h2i = vec![0; n + 1];
    for (i, h) in i2h.iter().copied().enumerate() {
        h2i[h] = i;
    }
    let mut ls = LazySegtree::<MaxMonoid>::new(n);
    let mut rs = 0;
    let mut h2v = vec![0; n + 1];
    for (h, i) in h2i.iter().copied().enumerate().skip(1) {
        // eprintln!("[{h:2}]{i:2}: {}", (0..n).map(|i| ls.get(i)).join(" "));
        if d < h {
            let j = h2i[h - d];
            let v = h2v[h - d];
            ls.set(j, v);
        }
        let max = ls.prod(i.saturating_sub(r)..=(n - 1).min(i + r));
        // eprintln!("{:?} {max}", i.saturating_sub(r)..=(n - 1).min(i + r));
        let v = max + 1;
        rs = rs.max(v);
        h2v[h] = v;
    }
    // eprintln!("  : {}", (0..n).map(|i| ls.get(i)).join(" "));
    rs -= 1;
    println!("{rs}");
}
