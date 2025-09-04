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
        aa: [usize; n],
        q: usize,
        llrr: [(usize, usize); q],
    };
    let mut ss = vec![0usize; 2];
    for (al, ar) in aa[1..].iter().copied().tuples() {
        let last = ss.last().unwrap();
        ss.extend([last + ar - al; 2]);
    }
    // eprintln!("{aa:?}");
    // eprintln!("{ss:?}");
    for (l, r) in llrr {
        let i = aa.partition_point(|&a| a <= l);
        let j = aa.partition_point(|&a| a <= r);
        // eprintln!("{l} {r} | {i} {j} {} {}", ss[i], ss[j]);
        let mut add = ss[j];
        if j.is_even() {
            add -= aa[j] - r;
        }
        let mut sub = ss[i];
        // rs -= ss[i.saturating_sub(1)];
        if i.is_even() {
            sub -= aa[i] - l;
        }
        let rs = add - sub;
        println!("{rs}");
    }
}
