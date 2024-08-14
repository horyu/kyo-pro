#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        mut aa: [Usize1; n],
    };
    // https://atcoder.jp/contests/abc296/editorial/6135
    for _ in 0..30 {
        aa = aa.iter().copied().map(|a| aa[a]).collect_vec();
    }
    let rs = aa.into_iter().unique().count();
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
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
