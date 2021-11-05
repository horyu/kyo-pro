#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use petgraph::graph::UnGraph;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbtt: [(Usize1, Usize1, usize); m]
    };
    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabbtt.iter().map(|&(i, j, w)| (nodes[i], nodes[j], w)));
    let rs = nodes
        .iter()
        .map(|&i| {
            petgraph::algo::dijkstra(&g, i, None, |e| *e.weight())
                .into_iter()
                .map(|(_, w)| w)
                .max()
                .unwrap()
        })
        .min()
        .unwrap();
    println!("{}", rs);
}
