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
        rrcc: [(usize, usize); n],
    };
    let mut r_max = 0;
    let mut r_min = !0;
    let mut c_max = 0;
    let mut c_min = !0;
    for (r, c) in rrcc {
        r_max = r_max.max(r);
        r_min = r_min.min(r);
        c_max = c_max.max(c);
        c_min = c_min.min(c);
    }
    let dr = r_max - r_min;
    let dc = c_max - c_min;
    let rs = dr.max(dc).div_ceil(2);
    println!("{rs}");
}
