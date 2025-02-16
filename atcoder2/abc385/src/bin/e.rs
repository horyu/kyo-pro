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
        uuvv: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let cc = g.iter().map(|v| v.len()).collect_vec();
    for ii in g.iter_mut() {
        ii.sort_unstable_by_key(|&v| cc[v]);
    }
    // ある頂点とx個の辺があり、接続された頂点はy個の辺を持つ
    // xy <= n - 1
    let mut rs = !0;
    for i in 0..n {
        // 頂点iを中心とする
        for (idx, j) in g[i].iter().copied().enumerate() {
            let x = cc[i] - idx;
            let y = cc[j];
            rs = rs.min(n - 1 - x * y);
        }
    }
    println!("{rs}");
}
