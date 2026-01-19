#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut hh: [usize; n],
        bb: [usize; m],
    };
    hh.sort_unstable();
    let mut bts = BTreeSet::from_iter(bb.into_iter().enumerate().map(|(i, b)| (b, i)));
    let mut cnt = 0;
    for h in hh.into_iter().rev() {
        if let Some(&k) = bts.range((h, 0)..).next() {
            cnt += 1;
            bts.remove(&k);
        }
    }
    let rs = ["No", "Yes"][(k <= cnt) as usize];
    println!("{rs}");
}
