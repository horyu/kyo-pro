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
        k: usize,
        mut aa: [isize; n],
    };
    aa.sort_unstable();
    let mut rs = isize::MAX;
    for vv in aa.windows(n - k) {
        let first = vv[0];
        let last = vv[n - k - 1];
        rs = rs.min(last - first);
    }
    println!("{rs}");
}
