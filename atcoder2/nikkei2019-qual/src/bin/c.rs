#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aabb: [(isize, isize); n],
    };
    aabb.sort_unstable_by_key(|ab| R(ab.0 + ab.1));
    let mut x = 0;
    let mut y = 0;
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        if i % 2 == 0 {
            x += a;
        } else {
            y += b;
        }
    }
    let rs = x - y;
    println!("{rs}");
}
