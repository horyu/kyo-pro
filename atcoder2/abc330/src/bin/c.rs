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
        d: usize,
    };
    let mut rs = usize::MAX;
    for x in 0..=(d.sqrt() + 1) {
        let xx = x.pow(2);
        let y = (xx.abs_diff(d)).sqrt();
        rs = rs
            .min((xx + y.pow(2)).abs_diff(d))
            .min((xx + (y + 1).pow(2)).abs_diff(d))
    }
    println!("{rs}");
}
