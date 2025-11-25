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
        h: usize,
        w: usize,
        mut ss: [Chars; h],
    };
    let (tx, ty) = ss
        .iter()
        .enumerate()
        .find_map(|(i, s)| {
            s.iter()
                .enumerate()
                .find_map(|(j, &c)| (c == 'T').then_some((i, j)))
        })
        .unwrap();
    ss[tx][ty] = '.';
    let mut qq = VecDeque::new();
    qq.push_back((ss.clone(), 0));
    let mut checked = HashSet::new();
    checked.insert(ss);
    while let Some((ss, d)) = qq.pop_front() {
        // TOOD
    }
    println!("-1");
}
