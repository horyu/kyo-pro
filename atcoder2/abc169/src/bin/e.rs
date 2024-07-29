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
        aabb: [(usize, usize); n],
    };
    // https://blog.hamayanhamayan.com/entry/2020/06/01/210806
    let aa = aabb.iter().map(|ab| ab.0).sorted_unstable().collect_vec();
    let bb = aabb.iter().map(|ab| ab.1).sorted_unstable().collect_vec();
    if n.is_even() {
        // 0.5刻み
        let l = aa[n / 2] + aa[n / 2 - 1];
        let r = bb[n / 2] + bb[n / 2 - 1];
        println!("{}", r - l + 1);
    } else {
        let l = aa[n / 2];
        let r = bb[n / 2];
        println!("{}", r - l + 1);
    }
}
