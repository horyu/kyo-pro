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
        q: usize,
        llrr: [(Usize1, Usize1); q],
    };
    let mut vv = vec![false; n + 1];
    for (l, r) in llrr {
        vv[l] = !vv[l];
        vv[r + 1] = !vv[r + 1];
    }
    let mut rs = String::new();
    let mut tf = false;
    for &v in &vv[..n] {
        tf ^= v;
        rs.push(if tf { '1' } else { '0' });
    }
    println!("{rs}");
}
