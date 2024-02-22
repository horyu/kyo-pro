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
        xxyyhh: [(usize, usize, usize); n],
    };
    let (x1, y1, h1) = xxyyhh.iter().copied().find(|&(_, _, h)| 0 < h).unwrap();
    for cx in 0usize..=100 {
        for cy in 0usize..=100 {
            let ch = h1 + cx.abs_diff(x1) + cy.abs_diff(y1);
            if xxyyhh.iter().all(|&(x, y, h)| h == ch.saturating_sub(cx.abs_diff(x) + cy.abs_diff(y))) {
                println!("{cx} {cy} {ch}");
                return;
            }
        }
    }
}
