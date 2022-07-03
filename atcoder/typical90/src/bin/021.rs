#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::graph::Graph;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aabb: [(Usize1, Usize1); m],
    };
    aabb.sort_unstable();
    aabb.dedup();

    let mut g = Graph::<(), ()>::new();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabb.iter().map(|&(i, j)| (nodes[i], nodes[j])));

    let mut rs = 0usize;
    for vv in petgraph::algo::kosaraju_scc(&g) {
        rs += vv.len() * (vv.len().saturating_sub(1)) / 2
    }
    println!("{rs}");
}
