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
        k: usize,
        aa: [usize; n],
    };
    let mut bts = BTreeSet::from_iter([0]);
    let mut cnt = 0;
    while let Some(base) = bts.pop_first() {
        if cnt == k {
            println!("{base}");
            return;
        }
        cnt += 1;
        for &a in &aa {
            bts.insert(base + a);
        }
    }
}
