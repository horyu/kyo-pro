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
        aabb: [(usize, usize); k],
    };
    let mut vv = vec![0; n + 1];
    let mut rrss = vec![];
    for (a, b) in aabb {
        vv[a] += 1;
        if vv[a] == m {
            rrss.push(a);
        }
    }
    let rs = rrss.into_iter().join(" ");
    println!("{rs}");
}
