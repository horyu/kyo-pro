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
        xxrrhh: [(usize, usize, usize); n],
        aabb: [(usize, usize); q],
    };
    // 高さ1の円錐台の体積
    let cal = |r1: f64, r2: f64| {
        std::f64::consts::PI / 3.0 * (r1 * r1 + r1 * r2 + r2 * r2)
    };
    let mut vv = vec![0.0; 1e4 as usize + 1];
    for (x, r, h) in xxrrhh {
        for hh in 0..h {
            let r1 = (r * (h - hh)) as f64 / h as f64;
            let r2 = (r * (h - hh - 1)) as f64 / h as f64;
            vv[x + hh] += cal(r1, r2);
        }
    }
    let ww = chain!([0.0], vv).cumsum::<f64>().collect_vec();
    for (a, b) in aabb {
        println!("{}", ww[b] - ww[a]);
    }
}
