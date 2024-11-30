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
        aacc: [(usize, char); n],
    };
    let mut rs = 0;
    let mut l = aacc
        .iter()
        .find_map(|ac| (ac.1 == 'L').then_some(ac.0))
        .unwrap_or_default();
    let mut r = aacc
        .iter()
        .find_map(|ac| (ac.1 == 'R').then_some(ac.0))
        .unwrap_or_default();
    for (a, c) in aacc {
        if c == 'L' {
            rs += a.abs_diff(l);
            l = a;
        } else {
            rs += a.abs_diff(r);
            r = a;
        }
    }
    println!("{rs}");
}
