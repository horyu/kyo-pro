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
        xxyycc: [(isize, isize, isize); n],
    };
    const MAX: isize = 1e5 as isize;
    let mut p = (-MAX..=MAX)
        .min_by_key(|&tmp| {
            xxyycc
                .iter()
                .map(|&(x, _, c)| (x - tmp).abs() * c)
                .max()
                .unwrap()
        })
        .unwrap() as f64;
    let mut q = (-MAX..=MAX)
        .min_by_key(|&tmp| {
            xxyycc
                .iter()
                .map(|&(_, y, c)| (y - tmp).abs() * c)
                .max()
                .unwrap()
        })
        .unwrap() as f64;
    dbg!(p, q);
    // 三部探索
    let mut l = p - 1.0;
    let mut r = p + 1.0;
    for _ in 0..64 {
        let m1 = (l * 2.0 + r) / 3.0;
        let m2 = (l + r * 2.0) / 3.0;
        let f1 = xxyycc
            .iter()
            .map(|&(x, _, c)| (x as f64 - m1).abs() * c as f64)
            .fold(0.0f64, |acc, v| acc.max(v));
        let f2 = xxyycc
            .iter()
            .map(|&(x, _, c)| (x as f64 - m2).abs() * c as f64)
            .fold(0.0f64, |acc, v| acc.max(v));
        if f1 < f2 {
            r = m2;
        } else {
            l = m1;
        }
    }
    p = l;
    let mut l = q - 1.0;
    let mut r = q + 1.0;
    for _ in 0..64 {
        let m1 = (l * 2.0 + r) / 3.0;
        let m2 = (l + r * 2.0) / 3.0;
        let f1 = xxyycc
            .iter()
            .map(|&(_, y, c)| (y as f64 - m1).abs() * c as f64)
            .fold(0.0f64, |acc, v| acc.max(v));
        let f2 = xxyycc
            .iter()
            .map(|&(_, y, c)| (y as f64 - m2).abs() * c as f64)
            .fold(0.0f64, |acc, v| acc.max(v));
        if f1 < f2 {
            r = m2;
        } else {
            l = m1;
        }
    }
    q = l;
    dbg!(p, q);
    let rs = xxyycc
        .iter()
        .map(|&(xx, yy, cc)| {
            ((xx as f64 - p).abs() * cc as f64).max((yy as f64 - q).abs() * cc as f64)
        })
        .fold(0.0f64, |acc, v| acc.max(v));
    println!("{rs}");
}
