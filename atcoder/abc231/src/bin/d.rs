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
    let mut hm = HashMap::new();
    for &(a, b) in &aabb {
        *hm.entry(a).or_insert(0) += 1;
        *hm.entry(b).or_insert(0) += 1;
    }
    if hm.values().any(|v| *v >= 3) {
        println!("No");
        return;
    }

    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabb.iter().map(|&(i, j)| (nodes[i], nodes[j])));
    let tf = petgraph::algo::is_cyclic_undirected(&g);
    println!("{}", ["No", "Yes"][!tf as usize]);
}
