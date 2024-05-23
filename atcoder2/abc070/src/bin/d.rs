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

fn main() {
    input! {
        n: usize,
        aabbcc: [(Usize1, Usize1, usize); n - 1],
        q: usize,
        k: Usize1,
        xxyy: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    for (u, v, c) in aabbcc.iter().copied() {
        g[u].push((v, c));
        g[v].push((u, c));
    }
    let mut dd = vec![0; n];
    let mut qq = VecDeque::new();
    qq.push_back((k, !0));
    while let Some((q, from)) = qq.pop_front() {
        for (next, c) in g[q].iter().copied() {
            if next == from {
                continue;
            }
            dd[next] = dd[q] + c;
            qq.push_back((next, q));
        }
    }
    // eprintln!("{dd:?}");
    for (x, y) in xxyy.iter().copied() {
        let rs = dd[x] + dd[y];
        println!("{rs}");
    }
}
