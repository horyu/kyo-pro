#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut ccdd = aa.into_iter().map(|a| (a, 0usize)).collect_vec();
    let mut rs = 0usize;
    for i in (0..=31).rev() {
        let target = 1usize << i;
        let mut new_ccdd = vec![];
        let mut sums = vec![];
        for &(c, d) in &ccdd {
            if 0 < c & target {
                sums.push(d);
                new_ccdd.push((c, d));
            } else {
                let new_d = d + target - (c & (target - 1));
                sums.push(new_d);
                new_ccdd.push((target, new_d));
            }
        }
        let sum = sums.into_iter().sorted_unstable().take(k).sum::<usize>();
        if sum <= m {
            rs |= target;
            ccdd = new_ccdd;
        }
    }
    println!("{rs}");
}
