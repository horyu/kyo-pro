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
        m: usize,
        aa: [usize; n],
    };
    let bb = aa
        .iter()
        .sorted_unstable()
        .dedup()
        .map(|a| a / 2)
        .collect_vec();
    let mut lcm = 1usize;
    for b in bb.iter() {
        lcm = lcm.lcm(b);
    }
    if bb.iter().copied().any(|b| (lcm / b).is_even()) {
        println!("0");
        return;
    }

    let rs = (m / lcm + 1) / 2;
    println!("{rs}");
}
