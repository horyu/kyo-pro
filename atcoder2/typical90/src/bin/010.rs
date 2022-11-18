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
        ccpp: [(Usize1, usize); n],
        q: usize,
        llrr: [(Usize1, Usize1); q],
    };
    let mut ss = vec![vec![0; n + 1]; 2];
    for (i, (c, p)) in ccpp.into_iter().enumerate() {
        for ci in 0..2 {
            ss[ci][i + 1] = ss[ci][i];
            if c == ci {
                ss[ci][i + 1] += p;
            }
        }
    }
    for (l, r) in llrr {
        let rs = (0..2).map(|c| ss[c][r + 1] - ss[c][l]).join(" ");
        println!("{rs}");
    }
}
