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
        q: usize,
        xxyy: [(usize, usize); q],
    };
    let mut ft = ac_library::FenwickTree::new(n + 1, 0isize);
    for i in 1..=n {
        ft.add(i, 1);
    }
    for (x, y) in xxyy {
        let sum = ft.sum(..=x);
        if 0 < sum {
            ft.add(0, -sum);
            ft.add(y, sum);
        }
        let rs = sum.max(0);
        println!("{rs}");
        eprintln!("{}", (0..=n).map(|i| ft.sum(i..=i)).join(" "));
    }
}
