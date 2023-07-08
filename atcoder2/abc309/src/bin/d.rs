#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut g1 = vec![vec![]; n1];
    let mut g2 = vec![vec![]; n2];
    for (u, v) in aabb.iter().copied() {
        if u < n1 {
            g1[u].push((v, 1));
            g1[v].push((u, 1));
        } else {
            g2[u - n1].push((v - n1, 1));
            g2[v - n1].push((u - n1, 1));
        }
    }
    let mut rs = 1;
    rs += pathfinding::prelude::dijkstra_all(&0usize, |&i| g1[i].iter().copied())
        .into_values()
        .map(|(_, c)| c)
        .max()
        .unwrap_or_default();
    rs += pathfinding::prelude::dijkstra_all(&(n2 - 1), |&i| g2[i].iter().copied())
        .into_values()
        .map(|(_, c)| c)
        .max()
        .unwrap_or_default();
    println!("{rs}");
}
