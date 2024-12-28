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
        s: f64,
        t: f64,
        aabbccdd: [((f64, f64), (f64, f64)); n],
    };
    let mut rs = f64::MAX;
    fn f(from: (f64, f64), to: (f64, f64), v: f64) -> f64 {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        (dx * dx + dy * dy).sqrt() / v
    }
    // 線分の順番で全探索
    for aabbccdd in aabbccdd.into_iter().permutations(n) {
        let mut vv = [0.0f64; 2];
        let mut pre = [(0.0, 0.0); 2];
        // eprintln!("{aabbccdd:?}");
        for (ab, cd) in aabbccdd.iter().copied() {
            let mut new_vv = [f64::MAX; 2];
            // from -> mid -> to
            for (idx, mid, to) in [(1, ab, cd), (0, cd, ab)] {
                for (v, from) in izip!(vv, pre) {
                    new_vv[idx] = new_vv[idx].min(v + f(from, mid, s) + f(mid, to, t));
                }
            }
            vv = new_vv;
            pre = [ab, cd];
        }
        // eprintln!("{vv:?} {pre:?}");
        rs = rs.min(vv[0].min(vv[1]));
    }
    println!("{rs}");
}
