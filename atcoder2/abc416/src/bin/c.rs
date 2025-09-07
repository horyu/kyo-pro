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
        k: usize,
        x: Usize1,
        ss: [String; n],
    };
    let mut vv = vec![];
    for ss in (0..k).map(|_| &ss).multi_cartesian_product() {
        let mut tmp = String::new();
        for s in ss {
            tmp.push_str(s);
        }
        vv.push(tmp);
    }
    vv.sort_unstable();
    let rs = &vv[x];
    println!("{rs}");
}
