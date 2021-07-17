#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::{graph::Graph, Undirected};
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [isize; n],
        bb: [isize; n],
        ccdd: [(Usize1, Usize1); m]
    };
    // https://qiita.com/41semicolon/items/356b7f03cfd138378543
    let mut g = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(ccdd.iter().map(|&(i, j)| (nodes[i], nodes[j])));
    for vv in petgraph::algo::kosaraju_scc(&g) {
        let indexes = vv.iter().map(|v| v.index()).collect_vec();
        if indexes.iter().map(|i| aa[*i]).sum::<isize>()
            != indexes.iter().map(|i| bb[*i]).sum::<isize>()
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
