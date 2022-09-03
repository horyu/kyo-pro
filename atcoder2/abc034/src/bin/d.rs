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
        k: usize,
        wwpp: [(f64, f64); n],
    };
    let mut ok = 0.0;
    let mut ng = 100.1;
    for _ in 0..1000 {
        let m = (ok + ng) / 2.0;
        let sum = wwpp
            .iter()
            .map(|&(w, p)| w * (p - m))
            .sorted_unstable_by(|x, y| x.partial_cmp(y).unwrap())
            .rev()
            .take(k)
            .sum::<f64>();
        if 0.0 <= sum {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{ok}");
}
