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
        eprintln!("{tt:?}");
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
        // dx^2 + dy^2 が最小となるのは両端または極小点
        let mut rs = f(0.0).min(f(tt[0])).min(f(tt[1]));
        eprintln!("{} {} {}", f(0.0), f(tt[0]), f(tt[1]));
        // 0 ~ tt[0] の範囲に極小点が auu + bu + c としたとき u = -b/2a
        let a = ((gx0 - sx0) / tt[0] - (gx1 - sx1) / tt[1]).powi(2)
            + ((gy0 - sy0) / tt[0] - (gy1 - sy1) / tt[1]).powi(2);
        let b = 2.0
            * ((sx0 - sx1) * ((gx0 - sx0) / tt[0] - (gx1 - sx1) / tt[1])
                + (sy0 - sy1) * ((gy0 - sy0) / tt[0] - (gy1 - sy1) / tt[1]));
        if 1e-12 < a.abs() {
            let u = -b / (2.0 * a);
            if 0.0 <= u && u <= tt[0] {
                rs = rs.min(f(u));
                eprintln!("extreme: {}", f(u));
            }
        }
        // tt[0] ~ tt[1] の範囲で線分と点の距離が最小となる場合
        // TOOD

        println!("{rs}");
    }
}
