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
        k: usize,
    };
    let pow = 2usize.pow(n as u32);
    let mut bb = vec![k / pow; pow];
    let mut kk = k % pow;
    // 可能な限り分散するようにkkを分配する
    // 2の冪乗ごとに分配するイメージ
    // 12345678
    // 1 3 2 4
    //  5 7 6 8
    todo!();
    // println!("{rs}");
}
