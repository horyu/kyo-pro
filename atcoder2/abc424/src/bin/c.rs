#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n],
    };
    let mut g = vec![vec![]; n + 1];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        if (a, b) == (0, 0) {
            g[n].push(i);
            continue;
        }
        g[a - 1].push(i);
        g[b - 1].push(i);
    }
    let rs = pathfinding::prelude::bfs_reach(n, |&s| g[s].iter().copied()).count() - 1;
    println!("{rs}");
}
