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
        n: usize,
        k: usize,
        aabb: [(Usize1, Usize1); n - 1],
        vv: [Usize1; k],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut ttff = vec![false; n];
    for v in vv.iter().copied() {
        ttff[v] = true;
    }
    dfs(&g, &mut ttff, vv[0], !0);
    let rs = ttff.into_iter().filter(|&tf| tf).count();
    println!("{rs}");
}

// ある頂点iから頂点群vvを含まれる頂点への経路をマークする
fn dfs(g: &[Vec<usize>], vv: &mut Vec<bool>, i: usize, p: usize) -> bool {
    let mut rs = vv[i];
    for j in g[i].iter().copied() {
        if j == p {
            continue;
        }
        if dfs(g, vv, j, i) {
            vv[j] = true;
            rs = true;
        }
    }
    if rs {
        vv[i] = true;
    }
    rs
}
