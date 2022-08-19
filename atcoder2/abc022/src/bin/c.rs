#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        uuvvll: [(Usize1, Usize1, usize); m],
    };
    let mut g = petgraph::Graph::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    let edges = uuvvll
        .iter()
        .map(|&(u, v, l)| g.add_edge(nodes[u], nodes[v], l))
        .collect_vec();
    let mut ii = vec![];
    for (i, &(u, v, l)) in uuvvll.iter().enumerate() {
        if u == 0 {
            ii.push(i);
        }
    }
    let mut rs = std::usize::MAX;
    for i in ii.into_iter().rev() {
        let (a, b, l) = uuvvll[i];
        g.remove_edge(edges[i]);
        // dbg!(i,a,b, petgraph::algo::dijkstra(&g, nodes[0], Some(nodes[b]), |e| *e.weight()));
        let hm = petgraph::algo::dijkstra(&g, nodes[0], Some(nodes[b]), |e| *e.weight());
        if let Some(&w) = hm.get(&nodes[b]) {
            rs = rs.min(l + w);
        }
        g.add_edge(nodes[a], nodes[b], l);
    }
    if rs == std::usize::MAX {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
