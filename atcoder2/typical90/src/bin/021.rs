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
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = petgraph::Graph::<(), ()>::new();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabb.iter().unique().map(|&(i, j)| (nodes[i], nodes[j])));

    let rs = petgraph::algo::tarjan_scc(&g)
        .into_iter()
        .map(|vv| vv.len())
        .fold(0, |acc, len| acc + len * (len.saturating_sub(1)) / 2);
    println!("{rs}");
}
