#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use counter::Counter;
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
        c: usize,
        ddd: [[usize; c]; c],
        aaa: [[Usize1; n]; n],
    };
    let mut counts = vec![Counter::<usize>::new(); 3];
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().copied().enumerate() {
            counts[(i + j) % 3][&a] += 1;
        }
    }
    let costs = counts
        .into_iter()
        .map(|counter| {
            (0..c)
                .map(|aft| {
                    counter
                        .iter()
                        .map(|(&bef, &cnt)| cnt * ddd[bef][aft])
                        .sum::<usize>()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut rs = std::usize::MAX;
    for ii in (0..c).permutations(3) {
        let mut tmp = 0;
        for (i, a) in ii.iter().copied().enumerate() {
            tmp += costs[i][a];
        }
        rs = rs.min(tmp);
    }
    println!("{rs}");
}
