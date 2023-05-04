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
    let lsum = chain!([0], ll.iter().copied())
        .cumsum::<usize>()
        .collect_vec();
    let rsum = chain!([0], rr.iter().copied())
        .cumsum::<usize>()
        .collect_vec();
    let ww = BTreeSet::from_iter(ww);
    let mut rs = !0usize;
    // eprintln!("{}", hh.iter().join(" "));
    // eprintln!("{}", lsum.iter().join(" "));
    // eprintln!("{}", rsum.iter().join(" "));
    for (i, h) in hh.iter().copied().enumerate().step_by(2) {
        let d = [ww.range(..=h).next_back(), ww.range(h..).next()]
            .into_iter()
            .filter_map(|wopt| wopt.map(|w| w.abs_diff(h)))
            .min()
            .unwrap();
        let j = i / 2;
        rs = rs.min(d + lsum[j] + rsum[n / 2 - j]);
    }
    println!("{rs}");
}
