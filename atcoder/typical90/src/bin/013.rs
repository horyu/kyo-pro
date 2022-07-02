#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::graph::UnGraph;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m]
    };
    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabbcc.iter().map(|&(i, j, w)| (nodes[i], nodes[j], w)));
    
    // 1とNからダイクストラ
    let mut rs = vec![0; n];
    for start in [0, n - 1] {
        for (to, w) in petgraph::algo::dijkstra(&g, nodes[start], None, |e| *e.weight()) {
            rs[to.index()] += w;
        }
    }
    println!("{}", rs.into_iter().join("\n"));
}
