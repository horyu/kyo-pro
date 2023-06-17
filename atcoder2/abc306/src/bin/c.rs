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
        aa: [Usize1; 3 * n],
    };
    let mut a2ii = vec![vec![]; n];
    for (i, a) in aa.iter().copied().enumerate() {
        a2ii[a].push(i);
    }
    let rs = (1..=n).sorted_unstable_by_key(|a| a2ii[a - 1][1]).join(" ");
    println!("{rs}");
}
