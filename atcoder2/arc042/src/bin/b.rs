#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: f64,
        b: f64,
        n: usize,
        xxyy: [(f64, f64); n],
    };
    let mut rs = std::f64::MAX;
    // https://qiita.com/kkdd/items/b3c5e06798e59fe2768e
    let d = |xi: f64, yi: f64, xj: f64, yj: f64| -> f64 {
        ((a - xi) * (yi - yj) - (b - yi) * (xi - xj)).abs()
            / ((xi - xj).powi(2) + (yi - yj).powi(2)).sqrt()
    };
    for ((xi, yi), (xj, yj)) in xxyy.into_iter().cycle().tuple_windows().take(n) {
        rs = rs.min(d(xi, yi, xj, yj));
    }
    println!("{rs}");
}
