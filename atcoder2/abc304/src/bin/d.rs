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
        w: usize,
        h: usize,
        n: usize,
        ppqq: [(usize, usize); n],
        a: usize,
        aa: [usize; a],
        b: usize,
        bb: [usize; b],
    };
    let xx = chain!([0, w], aa).sorted_unstable().collect_vec();
    let yy = chain!([0, h], bb).sorted_unstable().collect_vec();
    let mut max = 0;
    let mut hm = HashMap::new();
    for (p, q) in ppqq {
        let i = xx.partition_point(|&x| x < p);
        let j = yy.partition_point(|&y| y < q);
        let e = hm.entry((i, j)).or_insert(0);
        *e += 1;
        max = max.max(*e);
    }
    let min = if hm.len() == (a + 1) * (b + 1) {
        hm.values().min().copied().unwrap()
    } else {
        0
    };
    println!("{min} {max}");
}
