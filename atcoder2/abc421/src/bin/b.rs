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
        x: usize,
        y: usize,
    };
    let f = |mut n: usize| -> usize {
        while 0 < n && n.is_multiple_of(10) {
            n /= 10;
        }
        let mut v = 0;
        while 0 < n {
            v *= 10;
            v += n % 10;
            n /= 10;
        }
        v
    };
    let mut aa = [0; 10];
    aa[0] = x;
    aa[1] = y;
    for i in 2..10 {
        aa[i] = f(aa[i - 1] + aa[i - 2]);
    }
    let rs = aa[9];
    println!("{rs}");
}
