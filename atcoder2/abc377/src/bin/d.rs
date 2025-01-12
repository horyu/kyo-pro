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
        llrr: [(usize, usize); n],
    };
    let mut ll_max = vec![0; m + 1];
    for (l, r) in llrr {
        ll_max[r] = ll_max[r].max(l);
    }
    let mut rs = 0;
    let mut l_max = 0;
    for r in 1..=m {
        l_max = l_max.max(ll_max[r]);
        rs += r - l_max;
    }
    println!("{rs}");
}
