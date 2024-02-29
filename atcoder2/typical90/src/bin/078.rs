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
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut less_counts = vec![0; n];
    for &(a, b) in &aabb {
        let min = a.min(b);
        let max = a.max(b);
        less_counts[max] += 1;
    }
    let rs = less_counts.into_iter().filter(|&c| c == 1).count();
    println!("{rs}");
}
