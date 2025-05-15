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
        k: usize,
    };
    let mut ft = ac_library::FenwickTree::new(n + 1, 0u128);
    for i in 0..k {
        ft.add(i, 1);
    }
    for i in k..=n {
        let v = ft.sum((i - k)..i) % 1_000_000_000;
        ft.add(i, v);
    }
    let rs = ft.sum(n..=n);
    println!("{rs}");
}
