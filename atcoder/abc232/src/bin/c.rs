#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
        ccdd: [(Usize1, Usize1); m],
    };
    use petgraph::graph::UnGraph;
    let mut g1 = UnGraph::<(), usize>::new_undirected();
    let nn1 = (0..n).map(|_| g1.add_node(())).collect_vec();
    g1.extend_with_edges(aabb.iter().map(|&(i, j)| (nn1[i], nn1[j])));
    let mut g2 = UnGraph::<(), usize>::new_undirected();
    let nn2 = (0..n).map(|_| g2.add_node(())).collect_vec();
    g2.extend_with_edges(ccdd.iter().map(|&(i, j)| (nn2[i], nn2[j])));

    let tf = petgraph::algo::is_isomorphic(&g1, &g2);
    println!("{}", ["No", "Yes"][tf as usize]);
}
