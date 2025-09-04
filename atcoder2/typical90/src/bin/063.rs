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
        h: usize,
        w: usize,
        ppp: [[usize; w]; h],
    };
    let mut rs = 0usize;
    for bb in 1u32..(1 << h) {
        let ii = (0..h).filter(|&i| (bb >> i) & 1 == 1).collect_vec();
        let mut counter = counter::Counter::<_>::new();
        for j in 0..w {
            if ii.iter().copied().map(|i| ppp[i][j]).all_equal() {
                counter[&ppp[ii[0]][j]] += 1;
            }
        }
        if let Some(&max) = counter.values().max() {
            rs = rs.max(max * ii.len());
        }
    }
    println!("{rs}");
}
