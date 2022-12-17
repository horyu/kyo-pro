#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::{matrix_graph::Zero, unionfind::UnionFind};
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        uuvvww: [(Usize1, Usize1, usize); n - 1],
    };
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uuvvww {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    qq.push_back((0, 0));
    pushed[0] = true;
    let mut rs = vec![0; n];
    while let Some((qi, qw)) = qq.pop_front() {
        for &(j, w) in &g[qi] {
            if pushed[j] {
                continue;
            }
            pushed[j] = true;
            rs[j] = (rs[qi].is_zero() ^ w.is_even()) as i32;
            let ww = qw + w;
            // eprintln!("{j}-{ww}");
            qq.push_back((j, ww));
        }
    }
    let rs = rs.iter().join("\n");
    println!("{rs}");
}
