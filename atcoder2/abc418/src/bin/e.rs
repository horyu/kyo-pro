#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        mut xxyy: [(isize, isize); n],
    };
    xxyy.sort_unstable();
    let hs = HashSet::<_>::from_iter(xxyy.iter().copied());
    let mut mm = multimap::MultiMap::new();
    for ((x1, y1), (x2, y2)) in xxyy.iter().copied().tuple_combinations() {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dl = dx * dx + dy * dy;
        let g = dx.gcd(&dy);
        mm.insert((dx / g, dy / g), dl);
    }
    let mut rs = 0;
    let mut sub = 0;
    for ((dx, dy), ddll) in mm {
        let len = ddll.len();
        rs += len * (len - 1) / 2;
        for size in ddll.into_iter().counts().into_values() {
            sub += size * (size - 1) / 2;
        }
    }
    rs -= sub / 2;
    println!("{rs}");
}
