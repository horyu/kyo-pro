#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        k: usize,
    };
    let mut qq = VecDeque::from_iter(1usize..=9);
    let mut cnt = 0;
    while let Some(q) = qq.pop_front() {
        cnt += 1;
        if cnt == k {
            println!("{q}");
            return;
        }
        for i in 0..(q % 10) {
            qq.push_back(q * 10 + i);
        }
    }
}
