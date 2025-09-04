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
        mut aabb: [(usize, usize); n],
    };
    aabb.sort_unstable();
    let mut tmp = 0;
    for (a, b) in aabb.iter().copied() {
        tmp += b;
    }

    if tmp <= k {
        println!("1");
        return;
    }
    for (a, b) in aabb.iter().copied() {
        tmp -= b;
        if tmp <= k {
            println!("{}", a + 1);
            return;
        }
    }
}
