#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

fn main() {
    input! {
        n: usize,
        k: usize,
        xxyy: [(isize, isize); n],
    };
    let mut xx = vec![];
    let mut yy = vec![];
    for &(x, y) in &xxyy {
        xx.push(x);
        yy.push(y);
    }
    xx.sort_unstable();
    yy.sort_unstable();
    let is_ok = |xrange: RangeInclusive<isize>, yrange: RangeInclusive<isize>| -> bool {
        k <= xxyy
            .iter()
            .filter(|&&(x, y)| xrange.contains(&x) && yrange.contains(&y))
            .count()
    };

    let mut rs = std::isize::MAX;
    for (&xl, &xr) in xx.iter().tuple_combinations() {
        for (&yd, &yu) in yy.iter().tuple_combinations() {
            if is_ok(xl..=xr, yd..=yu) {
                rs = rs.min((xr - xl) * (yu - yd));
            }
        }
        for (yd, yu) in yy.iter().map(|&y| (y, y + 1)) {
            if is_ok(xl..=xr, yd..=yu) {
                rs = rs.min((xr - xl) * (yu - yd));
            }
        }
    }
    for (&yd, &yu) in yy.iter().tuple_combinations() {
        for (&xl, &xr) in xx.iter().tuple_combinations() {
            if is_ok(xl..=xr, yd..=yu) {
                rs = rs.min((xr - xl) * (yu - yd));
            }
        }
        for (xl, xr) in xx.iter().map(|&x| (x, x + 1)) {
            if is_ok(xl..=xr, yd..=yu) {
                rs = rs.min((xr - xl) * (yu - yd));
            }
        }
    }
    println!("{rs}");
}
