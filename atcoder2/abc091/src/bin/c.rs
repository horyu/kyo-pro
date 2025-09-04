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
        aabb: [(usize, usize); n],
        mut ccdd: [(usize, usize); n],
    };
    let mut rs = 0usize;
    for (a, b) in aabb.into_iter().sorted_unstable().rev() {
        if let Some(j) = (0..ccdd.len())
            .filter(|&j| a < ccdd[j].0 && b < ccdd[j].1)
            .min_by_key(|&j| ccdd[j].1)
        {
            ccdd.swap_remove(j);
            rs += 1;
        }
    }
    println!("{rs}");
}
