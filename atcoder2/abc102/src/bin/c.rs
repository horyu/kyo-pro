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
        aa: [isize; n],
    };
    let cc = aa
        .iter()
        .copied()
        .enumerate()
        .map(|(i, a)| a - (i + 1) as isize)
        .sorted_unstable()
        .collect_vec();
    let b = if n.is_odd() {
        cc[n / 2]
    } else {
        (cc[n / 2 - 1] + cc[n / 2]) / 2
    };
    let rs = cc.iter().copied().map(|c| (c - b).abs()).sum::<isize>();
    println!("{rs}");
}
