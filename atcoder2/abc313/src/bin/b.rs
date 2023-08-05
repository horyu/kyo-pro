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
    let mut g = vec![vec![]; n];
    for (u, v) in aabb.iter().copied() {
        g[u].push(v);
    }
    for i in 0..n {
        let mut qq = VecDeque::new();
        let mut pushed = vec![false; n];
        qq.push_back(i);
        pushed[i] = true;
        while let Some(qi) = qq.pop_front() {
            for &qj in g[qi].iter() {
                if pushed[qj] {
                    continue;
                }
                pushed[qj] = true;
                qq.push_back(qj);
            }
        }
        if pushed.iter().all(|&tf| tf) {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
