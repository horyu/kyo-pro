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
        a: usize,
        x: f64,
        y: f64,
    };
    let mut btm = BTreeMap::new();
    btm.insert(0, 0.0);
    let rs = f(&mut btm, a, x, y, n);
    println!("{rs}");
}

fn f(memo: &mut BTreeMap<usize, f64>, a: usize, x: f64, y: f64, v: usize) -> f64 {
    if let Some(&rs) = memo.get(&v) {
        return rs;
    }
    let p = x + f(memo, a, x, y, v / a);
    let q = 6.0 / 5.0 * y + 1.0 / 5.0 * (2..=6).fold(0.0, |acc, i| acc + f(memo, a, x, y, v / i));
    let rs = p.min(q);
    memo.insert(v, rs);
    rs
}
