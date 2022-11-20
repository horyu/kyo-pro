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
        ss: [Bytes; n],
    };
    let mut ff = vec![];
    let mut tt = vec![];
    for s in ss {
        ff.push(s[0] - b'a');
        tt.push(*s.last().unwrap() - b'a');
    }
    // bit DP
    let mut dp = vec![0usize; 1 << n];
    for s in 1..(1 << n) {
        for c in 0..n {
            if 0 < 1 & (s >> c) {
                dp[s] |= (1 & !dp[s ^ 1 << c] >> tt[c]) << ff[c];
            }
        }
    }
    let rs = if dp.last().unwrap().is_zero() {
        "Second"
    } else {
        "First"
    };
    println!("{rs}");
}
