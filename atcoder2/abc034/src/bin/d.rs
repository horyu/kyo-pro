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
        k: usize,
        wwpp: [(f64, f64); n],
    };
    let mut ok = 0.0;
    let mut ng = 100.1;
    for i in 0..10000 {
        let m = (ok + ng) / 2.0;
        let sum = wwpp
            .iter()
            .copied()
            .map(|(w, p)| w * (p - m))
            .sorted_unstable_by(|a, b| b.partial_cmp(a).unwrap())
            .take(k)
            .sum::<f64>();
        if 0.0 <= sum {
            ok = m;
        } else {
            ng = m;
        }
    }
    let rs = ok;
    println!("{rs}");
}
