#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::{data::Element, graph::UnGraph};
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, isize); m]
    };
    let mut rs = 0;
    for &(_a, _b, c) in aabbcc.iter() {
        if c > 0 {
            rs += c;
        }
    }

    // クラスカル法
    let mut g = UnGraph::<(), isize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(
        aabbcc
            .iter()
            .map(|&(i, j, w)| (nodes[i], nodes[j], if w < 0 { 0 } else { w })),
    );

    for x in petgraph::algo::min_spanning_tree(&g) {
        if let Element::Edge {
            source: _,
            target: _,
            weight,
        } = x
        {
            rs -= weight;
        }
    }
    println!("{rs}");
}
