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
        t: usize,
        ccdd: [(usize, usize); t],
    };
    for (c, d) in ccdd {
        // 1<=x<=d
        // f(c,c+x) = c * 10^((c+x).ilog10() + 1) + c+x
        //          = k^2
        // c   = k^2 / 10^n
        // c+x = k^2 % 10^n
        // x = k^2 % 10^n - k^2 / 10^n
        // println!("{rs}");
    }
    let mut bts = BTreeSet::new();
    for i in 1..=24 {
        // eprintln!("{i}: {}", i * i % 100);
        bts.insert(i * i % 100);
    }
    dbg!(bts.len());
    dbg!(bts);
}
