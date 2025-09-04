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
    };
    let mut btm = BTreeMap::new();
    btm.insert(n, 1);
    let mut rs = 0usize;
    while let Some((k, v)) = btm.pop_last() {
        rs += k * v;
        for kk in [k / 2, k.div_ceil(2)] {
            if 1 < kk {
                *btm.entry(kk).or_insert(0) += v;
            }
        }
    }
    println!("{rs}");
}
