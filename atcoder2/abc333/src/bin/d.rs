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
    let rs = g[0]
        .iter()
        .copied()
        .map(|to| dfs(&g, to, 0))
        .sorted_unstable()
        .rev()
        .skip(1)
        .sum::<usize>()
        + 1;
    println!("{rs}");
}

fn dfs(g: &[Vec<usize>], to: usize, from: usize) -> usize {
    let mut rs = 1;
    for &next in g[to].iter() {
        if next == from {
            continue;
        }
        rs += dfs(g, next, to);
    }
    rs
}
