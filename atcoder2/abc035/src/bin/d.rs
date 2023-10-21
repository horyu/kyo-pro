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
        m: usize,
        t: usize,
        aa: [usize; n],
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut xx = vec![None; n];
    {
        let mut g = petgraph::Graph::new();
        let nodes = (0..n).map(|i| g.add_node(0)).collect_vec();
        for &(a, b, c) in &aabbcc {
            g.add_edge(nodes[a], nodes[b], c);
        }
        for (i, c) in petgraph::algo::dijkstra(&g, nodes[0], None, |e| *e.weight()) {
            xx[i.index()] = Some(c);
        }
    }
    let mut yy = vec![None; n];
    {
        let mut g = petgraph::Graph::new();
        let nodes = (0..n).map(|i| g.add_node(0)).collect_vec();
        for &(a, b, c) in &aabbcc {
            g.add_edge(nodes[b], nodes[a], c);
        }
        for (i, c) in petgraph::algo::dijkstra(&g, nodes[0], None, |e| *e.weight()) {
            yy[i.index()] = Some(c);
        }
    }
    let rs = izip!(aa, xx, yy)
        .into_iter()
        .map(|(a, x, y)| {
            if let (Some(x), Some(y)) = (x, y) {
                a * t.saturating_sub(x + y)
            } else {
                0
            }
        })
        .max()
        .unwrap();
    println!("{rs}");
}
