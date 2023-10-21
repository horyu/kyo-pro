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
        ss: [Bytes; n],
    };
    let ffee = ss
        .into_iter()
        .map(|s| (s[0] - b'a', s.last().copied().unwrap() - b'a'))
        .collect_vec();
    // bit DP
    let mut dp = vec![0; 1 << n];
    for s in 0..(1usize << n) {
        for c in 0..n {
            if 0 != 1 & (s >> c) {
                let (f, e) = ffee[c];
                dp[s] |= (1 & !dp[s ^ (1 << c)] >> e) << f;
            }
        }
    }
    let rs = ["Second", "First"][(dp.last().copied().unwrap() != 0) as usize];
    println!("{rs}");
}
