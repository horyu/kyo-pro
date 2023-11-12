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
        h: usize,
        w: usize,
        ppp: [[usize; w]; h],
    };
    let mut rs = 0;
    for bits in 1u32..(1 << h) {
        let mut counter = counter::Counter::<usize>::new();
        let ii = (0..h).filter(|&i| 0 < bits & (1 << i)).collect_vec();
        for j in 0..w {
            if ii.iter().copied().map(|i| ppp[i][j]).all_equal() {
                counter[&ppp[ii[0]][j]] += 1;
                rs = rs.max(counter[&ppp[ii[0]][j]] * ii.len());
            }
        }
    }
    println!("{rs}");
}
