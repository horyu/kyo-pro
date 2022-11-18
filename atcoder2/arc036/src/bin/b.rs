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
        hh: [usize; n],
    };
    let mut ll = vec![0usize; n];
    let mut rr = vec![0usize; n];
    for i in 0..(n - 1) {
        if hh[i] < hh[i + 1] {
            ll[i + 1] = ll[i] + 1;
        }
    }
    for i in (0..(n - 1)).rev() {
        if hh[i] > hh[i + 1] {
            rr[i] = rr[i + 1] + 1;
        }
    }
    // for i in 0..n {
    //     eprintln!("{i}:{} {}", ll[i], rr[i]);
    // }
    let rs = izip!(ll, rr).map(|(l, r)| l + r + 1).max().unwrap();
    println!("{rs}");
}
