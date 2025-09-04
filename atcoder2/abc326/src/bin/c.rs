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
        m: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let mut rs = 0;
    for (l, a) in aa.iter().copied().enumerate() {
        // a以上a+m未満の数を数える
        let r = aa.partition_point(|&x| x < a + m);
        rs = rs.max(r - l);
    }
    println!("{rs}");
}
