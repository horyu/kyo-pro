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
        aa: [isize; n],
    };
    let cc = aa
        .iter()
        .copied()
        .enumerate()
        .map(|(i, a)| a - i as isize)
        .sorted_unstable()
        .collect_vec();
    let len = cc.len();
    let mid = if len % 2 == 0 {
        (cc[len / 2 - 1] + cc[len / 2]) / 2
    } else {
        cc[len / 2]
    };
    let rs = cc.iter().copied().map(|c| (c - mid).abs()).sum::<isize>();
    println!("{rs}");
}
