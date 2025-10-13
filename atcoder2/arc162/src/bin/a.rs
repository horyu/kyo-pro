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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            pp: [Usize1; n],
        };
        let mut rs = 0;
        for (i, p) in pp.iter().copied().enumerate() {
            if pp[(i + 1)..].iter().all(|&x| p < x) {
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
