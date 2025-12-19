#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
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
        m: usize,
        ss: [Chars; n],
    };
    let aaa = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| u128::from(c == '.')).collect_vec())
        .collect_vec();
    let mut hs = HashSet::new();
    for x in 0..=(n - m) {
        for y in 0..=(n - m) {
            let mut bits = 0u128;
            for i in 0..m {
                for j in 0..m {
                    bits <<= 1;
                    bits |= aaa[x + i][y + j];
                }
            }
            hs.insert(bits);
        }
    }
    let rs = hs.len();
    println!("{rs}");
}
