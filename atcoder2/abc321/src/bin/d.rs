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
        p: usize,
        aa: [usize; n],
        mut bb: [usize; m],
    };
    bb.sort_unstable();

    let b_ss = chain!([0], bb.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    let mut rs = 0usize;
    for a in aa.iter().copied() {
        let i = bb.partition_point(|&b| a + b <= p);
        rs += i * a + b_ss[i] + (m - i) * p;
    }
    println!("{rs}");
}
