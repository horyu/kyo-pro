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
        xx: [isize; n],
    };
    let mut bts = BTreeSet::new();
    bts.insert(0);
    let mut sum = 1isize << 60;
    let mut x2d = BTreeMap::new();
    x2d.insert(0, sum);
    for x in xx {
        let l = *bts.range(..x).next_back().unwrap();
        let ld = *x2d.get(&l).unwrap();
        let mut xd = x - l;
        // left 側の距離を更新
        if x - l < ld {
            sum += (x - l) - ld;
            x2d.insert(l, x - l);
        }
        if let Some(&r) = bts.range(x..).next() {
            let rd = *x2d.get(&r).unwrap();
            // right 側の距離を更新
            if r - x < rd {
                sum += (r - x) - rd;
                x2d.insert(r, r - x);
            }
            xd = xd.min(r - x);
        }
        sum += xd;
        x2d.insert(x, xd);
        bts.insert(x);
        println!("{sum}");
    }
}
