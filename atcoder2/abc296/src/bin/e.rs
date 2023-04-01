#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    };
    let mut g = petgraph::Graph::new();
    let nodes = (0..n).map(|i| g.add_node(())).collect_vec();
    let mut rs = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        if i == a {
            rs += 1;
        } else {
            g.add_edge(nodes[i], nodes[a], ());
        }
    }
    for vv in petgraph::algo::kosaraju_scc(&g) {
        if 2 <= vv.len() {
            rs += vv.len();
        }
    }
    println!("{rs}");
}
