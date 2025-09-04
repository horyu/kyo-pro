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

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rrcc: [(Usize1, Usize1); q],
    };
    let mut x2yy = vec![BTreeSet::from_iter(0..w); h];
    let mut y2xx = vec![BTreeSet::from_iter(0..h); w];

    for (r, c) in rrcc {
        if x2yy[r].remove(&c) {
            y2xx[c].remove(&r);
            continue;
        }

        // LR削除
        if let Some(y) = x2yy[r].range(..c).next_back().copied() {
            y2xx[y].remove(&r);
            x2yy[r].remove(&y);
        }
        if let Some(y) = x2yy[r].range(c..).next().copied() {
            y2xx[y].remove(&r);
            x2yy[r].remove(&y);
        }
        // UD削除
        if let Some(x) = y2xx[c].range(..r).next_back().copied() {
            x2yy[x].remove(&c);
            y2xx[c].remove(&x);
        }
        if let Some(x) = y2xx[c].range(r..).next().copied() {
            x2yy[x].remove(&c);
            y2xx[c].remove(&x);
        }
    }

    let rs = x2yy.iter().map(|yy| yy.len()).sum::<usize>();
    println!("{rs}");
}
