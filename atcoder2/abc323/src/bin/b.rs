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
        ss: [Chars; n],
    };
    let cc = ss
        .into_iter()
        .map(|s| s.into_iter().filter(|&c| c == 'o').count())
        .collect_vec();
    let rs = (1..=n)
        .sorted_by_key(|i| std::cmp::Reverse(cc[i - 1]))
        .join(" ");
    println!("{rs}");
}
