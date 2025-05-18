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
        m: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![BTreeSet::new(); n];
    let mut cc = vec![0; n];
    for (u, v) in uuvv.iter().copied() {
        // g[u].insert(v);
        g[v].insert(u);
        cc[u] += 1;
        cc[v] += 1;
    }
    // 0:                      g[0][1..]
    // 1: g[1][..1]が存在する ? g[0][2..] & g[1][2..] : -1
    // 2: g[2][..2]が存在する ? g[..=2][3..] : -1
    let mut joined = BTreeSet::new();
    for k in 0..n {
        if g[k].range(..k).next().is_none() {

        }

        // println!("{rs}");
    }
}
