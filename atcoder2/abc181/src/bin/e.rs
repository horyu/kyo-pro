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
        n: usize,
        m: usize,
        mut hh: [usize; n],
        ww: [usize; m],
    };
    hh.sort_unstable();
    let ll = hh[..(n - 1)]
        .iter()
        .copied()
        .tuples()
        .map(|(x, y)| x.abs_diff(y))
        .collect_vec();
    let rr = hh[1..]
        .iter()
        .copied()
        .rev()
        .tuples()
        .map(|(x, y)| x.abs_diff(y))
        .collect_vec();

    let lsum = chain!([0], ll).cumsum::<usize>().collect_vec();
    let rsum = chain!([0], rr).cumsum::<usize>().collect_vec();

    // eprintln!("{}", lsum.iter().join(" "));
    // eprintln!("{}", rsum.iter().join(" "));
    // eprintln!("{}", hh.iter().join(" "));
    let mut rs = std::usize::MAX;
    for &w in &ww {
        let i = hh.partition_point(|&h| h <= w);
        // eprintln!("{w} {i}");
        if i.is_even() {
            if let Some(&h) = hh.get(i) {
                let tmp = lsum[i / 2] + h.abs_diff(w) + rsum[n / 2 - i / 2];
                // eprintln!("{w} {i} {h} {} {} {tmp}", lsum[i / 2], rsum[n / 2 - i / 2]);
                rs = rs.min(tmp);
            }
        } else {
            if let Some(&h) = hh.get(i - 1) {
                let tmp = lsum[i / 2] + h.abs_diff(w) + rsum[n / 2 - i / 2];
                // eprintln!("{w} {i} {h} {} {} {tmp}", lsum[i / 2], rsum[n / 2 - i / 2]);
                rs = rs.min(tmp);
            }
        }
    }
    println!("{rs}");
}
