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
        c: isize,
        mut xxvv: [(isize, isize); n],
    };
    let mut rs = 0;
    for _ in 0..2 {
        // 引き返し用btm
        let mut btm = BTreeMap::new();
        let mut crr = 0;
        let mut pos = c;
        let mut max = 0;
        for i in (0..n).rev() {
            let (x, v) = xxvv[i];
            crr += v - (pos - x);
            pos = x;
            if max < crr {
                max = crr;
                btm.insert(i, max);
            }
        }

        let mut crr = 0;
        let mut pos = 0;
        for i in 0..n {
            let (x, v) = xxvv[i];
            crr += v - (x - pos);
            pos = x;
            rs = rs.max(crr);
            // eprintln!("{i} {pos} {crr}");
            // 現地点から引き換えす
            if let Some((_, &jadd)) = btm.range((i + 1)..).min() {
                rs = rs.max(crr - pos + jadd);
            }
        }

        xxvv = xxvv
            .into_iter()
            .map(|(x, v)| (c - x, v))
            .rev()
            .collect_vec();
    }
    println!("{rs}");
}
