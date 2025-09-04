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
        m: usize,
        aa: [usize; n],
    };
    let mut vvv = vec![vec![]; m];
    let mut crr = 0;
    for (i, a) in aa.iter().cycle().enumerate().take(n * 2) {
        vvv[crr].push(i);
        crr = (crr + a) % m;
    }
    let mut rs = 0;
    let mut tmp = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        rs += vvv[tmp].partition_point(|&v| v < i + n) - vvv[tmp].partition_point(|&v| v <= i);
        tmp = (tmp + a) % m;
    }
    println!("{rs}");
}
