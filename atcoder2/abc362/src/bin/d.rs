#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use petgraph::visit::{EdgeRef, NodeRef};
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        uuvvbb: [(Usize1, Usize1, usize); m],
    };
    let mut g = petgraph::graph::Graph::new_undirected();
    let nodes = aa.iter().copied().map(|a| g.add_node(a)).collect_vec();
    for (u, v, b) in uuvvbb.iter().copied() {
        g.add_edge(nodes[u], nodes[v], b);
    }

    let hm = petgraph::algo::dijkstra(&g, nodes[0], None, |e| *e.weight() + g[e.target()]);
    let rs = nodes
        .iter()
        .copied()
        .skip(1)
        .map(|node| aa[0] + hm[&node])
        .join(" ");
    println!("{rs}");
}
