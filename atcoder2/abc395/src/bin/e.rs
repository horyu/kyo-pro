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
        x: usize,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n * 2];
    for i in 0..n {
        g[i].push((n + i, x));
        g[n + i].push((i, x));
    }
    for (u, v) in uuvv.iter().copied() {
        g[u].push((v, 1));
        g[n + v].push((n + u, 1));
    }
    if let Some((_, rs)) = pathfinding::prelude::dijkstra(
        &0,
        |&i| g[i].iter().copied(),
        |&i| i == n - 1 || i == 2 * n - 1,
    ) {
        println!("{rs}");
    }
}
