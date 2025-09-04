#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [Usize1; m],
        bb: [Usize1; m],
    };
    let mut g = petgraph::graph::UnGraph::<(), ()>::with_capacity(n, m);
    let nodes = (0..n).map(|i| g.add_node(())).collect_vec();
    let mut dsu = ac_library::Dsu::new(n);
    for (&a, &b) in izip!(&aa, &bb) {
        dsu.merge(a, b);
        g.add_edge(nodes[a], nodes[b], ());
    }
    let tf = dsu
        .groups()
        .into_iter()
        .all(|ii| petgraph::algo::is_bipartite_undirected(&g, nodes[ii[0]]));
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
