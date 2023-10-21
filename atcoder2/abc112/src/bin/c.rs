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
        xxyyhh: [(usize, usize, usize); n],
    };
    let (h_min, h_max) = match xxyyhh.iter().map(|xyh| xyh.2).minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(h) => (h, h),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    for i in 0usize..=100 {
        for j in 0usize..=100 {
            for k in h_max..=(h_min + 200) {
                if xxyyhh
                    .iter()
                    .copied()
                    .all(|(x, y, h)| k.saturating_sub(i.abs_diff(x) + j.abs_diff(y)) == h)
                {
                    println!("{i} {j} {k}");
                    return;
                }
            }
        }
    }
}
