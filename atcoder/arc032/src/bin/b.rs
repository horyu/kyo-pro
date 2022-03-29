#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::graph::UnGraph;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut g = UnGraph::<(), ()>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabb.iter().map(|&(i, j)| (nodes[i], nodes[j], ())));
    let rs = petgraph::algo::kosaraju_scc(&g).len() - 1;
    println!("{rs}");
}
