#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        s: Bytes,
    };
    let s = s.into_iter().map(|b| (b - b'a') as usize).collect_vec();
    // bbb:abb:bab:aab:bba:aba:baa:aaa
    let mut memo = vec![0usize; n + 1];
    memo[0] = 1;
    for i in 1..n {
        memo[i] = (0..i).fold(1, |acc, j| acc + memo[j]);
    }
    // eprintln!("{}", memo.iter().join(" "));
    let mut rs = 0;
    for (i, b) in s.iter().copied().enumerate() {
        rs += b * memo[i];
    }
    println!("{rs}");
}
