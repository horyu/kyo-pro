#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut xx: [isize; n],
    };
    xx.sort_unstable();
    let (pp, nn) = xx.iter().copied().partition::<Vec<_>, _>(|&x| 0 < x);
    // それぞれの最小3個と最大3個の合計12要素から3要素を選ぶ
    let mut yy = vec![];
    let plen = pp.len();
    for i in (0..3.min(plen))
        .chain(plen.saturating_sub(3)..plen)
        .unique()
    {
        yy.push(pp[i]);
    }
    let nlen = nn.len();
    for i in (0..3.min(nlen))
        .chain(nlen.saturating_sub(3)..nlen)
        .unique()
    {
        yy.push(nn[i]);
    }
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for (a, b, c) in yy.into_iter().tuple_combinations() {
        let v = (a + b + c) as f64 / (a * b * c) as f64;
        min = min.min(v);
        max = max.max(v);
    }
    println!("{min}\n{max}");
}
