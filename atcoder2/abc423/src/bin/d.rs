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
        k: isize,
        aabbcc: [(usize, usize, isize); n],
    };
    let mut btm = BTreeMap::new();
    let mut sum = 0isize;
    let mut now = 0usize;
    let mut rrss = vec![];
    for (a, b, c) in aabbcc.iter().copied() {
        while k < sum + c {
            let (t, v) = btm.pop_first().unwrap();
            now = t;
            sum += v;
        }
        if now <= a {
            now = a;
        }
        rrss.push(now);
        sum += c;
        *btm.entry(now + b).or_insert(0isize) -= c;
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
