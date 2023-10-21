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
        k: usize,
        aa: [usize; n],
    };
    let mut aacc = aa.into_iter().map(|a| (a, 0usize)).collect_vec();
    let mut rs = 0usize;
    for i in (0..=31).rev() {
        let target = 1usize << i;
        let mut new_aacc = vec![];
        let mut sums = vec![];
        for (a, c) in aacc.iter().copied() {
            if 0 < a & target {
                sums.push(c);
                new_aacc.push((a, c));
            } else {
                let new_c = c + target - (a & (target - 1));
                sums.push(new_c);
                new_aacc.push((target, new_c));
            }
        }
        let sum = sums.into_iter().sorted_unstable().take(k).sum::<usize>();
        if sum <= m {
            rs |= target;
            aacc = new_aacc;
        }
    }
    println!("{rs}");
}
