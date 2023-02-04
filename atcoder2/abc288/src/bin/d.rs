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
        k: usize,
        aa: [isize; n],
        q: usize,
        llrr: [(Usize1, Usize1); q],
    };
    if k == 1 {
        let rs = "Yes\n".repeat(q);
        println!("{rs}");
        return;
    }
    let mut ddd = vec![vec![0]; k];
    for (i, a) in aa.iter().copied().enumerate() {
        let last = ddd[i % k].last().copied().unwrap();
        ddd[i % k].push(last + a);
    }
    for (l, r) in llrr {
        let mut vv = aa[(r - k + 1)..r].iter().copied().collect_vec();
        for (j, v) in vv.iter_mut().enumerate() {
            // *v -= ddd[(r - k + 1) % m][(r - k + 1) / m] - ddd[(r - k + 1) % m][l / m];
            // *v += ddd[(r - k + 1 + j) / k * k + 1];
        }
        let rs = ["No", "Yes"][vv.into_iter().all(|v| v.is_zero()) as usize];
        println!("{rs}");
    }
}
