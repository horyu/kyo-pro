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
        k: usize,
        s: Chars,
    };
    let cc = s
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|x| x.1.count())
        .collect_vec();
    let len = cc.len();
    // すでにある区間は区間長 -1 だけスコアになる
    let mut rs = cc
        .iter()
        .copied()
        .fold(0, |acc, c| acc + c.saturating_sub(1));
    // 間にある区間を反転させると +2 できる
    let mut i = 1;
    let mut cnt = 0;
    while cnt < k && i < len {
        cnt += 1;
        // 区間 i-1 と i の間
        rs += 1;
        // 区間 i と i+1 の間
        if i + 1 < len {
            rs += 1;
        }
        i += 2;
    }
    println!("{rs}");
}
