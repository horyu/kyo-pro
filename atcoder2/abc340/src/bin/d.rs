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
        aabbxx: [(usize, usize, Usize1); n - 1],
    };
    let mut bh = BinaryHeap::new();
    bh.push((R(0), 0));
    let mut poped = vec![false; n];
    while let Some((R(crr), pos)) = bh.pop() {
        if poped[pos] {
            continue;
        }
        poped[pos] = true;
        if pos == n - 1 {
            println!("{crr}");
            return;
        }
        let (a, b, x) = aabbxx[pos];
        bh.push((R(crr + a), pos + 1));
        bh.push((R(crr + b), x));
    }
}
