#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use core::f64;
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
        // eprintln!("{tt:?}");
        // x = sx + (gx - sx) * u / tt[?]
        // y = sy + (gy - sy) * u / tt[?]
        let (sy0, sx0, gy0, gx0) = fff[0];
        let (sy1, sx1, gy1, gx1) = fff[1];
        let p0 = |u: f64| {
            if u <= tt[0] {
                (sx0 + (gx0 - sx0) * u / tt[0], sy0 + (gy0 - sy0) * u / tt[0])
            } else {
                (gx0, gy0)
            }
        };
        let p1 = |u: f64| (sx1 + (gx1 - sx1) * u / tt[1], sy1 + (gy1 - sy1) * u / tt[1]);
        let f = |u: f64| {
            let (px0, py0) = p0(u);
            let (px1, py1) = p1(u);
            (px0 - px1).hypot(py0 - py1)
        };

        // [0.0, tt[0]]: 二次関数部分 , [tt[0], tt[1]]: 点と線分の距離部分 に対して三分探索
        let mut rs = f64::INFINITY;
        for (mut l, mut r) in [(0.0, tt[0]), (tt[0], tt[1])] {
            for _ in 0..100 {
                let m1 = (2.0 * l + r) / 3.0;
                let m2 = (l + 2.0 * r) / 3.0;
                if f(m1) < f(m2) {
                    r = m2;
                } else {
                    l = m1;
                }
            }
            rs = rs.min(f((l + r) / 2.0));
        }
        println!("{rs:.15}");
    }
}
