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
    };
    let mut aa = vec![0; n + 1];
    let f = |mut x: usize| -> usize {
        let mut res = 0;
        while 0 < x {
            res += x % 10;
            x /= 10;
        }
        res
    };
    aa[0] = 1;
    for i in 1..=n {
        for j in 0..i {
            aa[i] += f(aa[j]);
        }
    }
    let rs = aa[n];
    println!("{rs}");
}
