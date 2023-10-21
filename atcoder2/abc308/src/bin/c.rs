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
    // ax / ax+bx = ay / ay+by
    // ax(ay+by) = ay(ax + bx)
    let ii = (0..n).sorted_unstable_by(|&x, &y| {
        let (ax, bx) = aabb[x];
        let (ay, by) = aabb[y];
        (ax * (ay + by))
            .cmp(&(ay * (ax + bx)))
            .reverse()
            .then(x.cmp(&y))
    });
    let rs = ii.into_iter().map(|i| i + 1).join(" ");
    println!("{rs}");
}
