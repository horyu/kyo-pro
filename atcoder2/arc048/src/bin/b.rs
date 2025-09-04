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
        n: usize,
        rrhh: [(Usize1, Usize1); n],
    };
    const MAX: usize = 100000;
    let mut ft = ac_library::FenwickTree::new(MAX, 0);
    let mut hm = HashMap::new();
    for (r, h) in rrhh.iter().copied() {
        ft.add(r, 1);
        let arr = hm.entry(r).or_insert([0, 0, 0]);
        arr[h] += 1;
    }
    for (r, h) in rrhh.iter().copied() {
        let arr = *hm.get(&r).unwrap();
        let win = ft.sum(0..r) + arr[(h + 1) % 3];
        let lose = ft.sum(r + 1..MAX) + arr[(h + 2) % 3];
        let draw = arr[h] - 1;
        println!("{win} {lose} {draw}");
    }
}
