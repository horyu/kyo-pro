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
        t: usize,
        // qq: [(f64, f64, f64, f64, f64, f64, f64, f64); t],
        qq: [[(f64, f64, f64, f64); 2]; t],
    };
    for mut fff in qq {
        // 0 ~ t0(一方が止まる) ~ t1(両方止まる)
        let mut tt = fff
            .iter()
            .copied()
            .map(|(sx, sy, gx, gy)| (sx - gx).hypot(sy - gy))
            .collect_vec();
        if tt[1] < tt[0] {
            fff.swap(0, 1);
            tt.swap(0, 1);
        }
        // x = sx + (gx - sx) * u
        // y = sy + (gy - sy) * u
        // dx = (sx0-sx1) + (gx0-sx0 - (gx1-sx1)) * u
        // dy = (sy0-sy1) + (gy0-sy0 - (gy1-sy1)) * u
        // dx^2 + dy^2 が最小となるのは両端または極小点
        // TODO
    }
    // println!("{rs}");
}
