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
        x: Usize1,
        hh: [usize; n],
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    let (_, rs) = dfs(&g, &hh, !0, x);
    println!("{rs}");
}

fn dfs(g: &[Vec<usize>], hh: &[usize], from: usize, to: usize) -> (bool, usize) {
    let mut tmp = 0;
    let mut should_check_children = false;
    for &next in &g[to] {
        if next == from {
            continue;
        }
        let (tf, nd) = dfs(g, hh, to, next);
        if tf || hh[next] == 1 {
            should_check_children = true;
            tmp += nd + 2;
        }
    }
    (
        should_check_children,
        [0, tmp][should_check_children as usize],
    )
}
